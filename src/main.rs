use rand::Rng;

use sfml::graphics::{
    Color, RectangleShape, RenderTarget, RenderWindow, Shape, Sprite, Text, Transformable, View,
};
use sfml::system::{Clock, Vector2f};
use sfml::window::{mouse, ContextSettings, Event, Key, Style};

mod platform;
use platform::{Platform, PlatformType};
mod state_stack;
use state_stack::{StateStack, StateType};
mod resource_manager;
use resource_manager::{FontIdentifiers, FontManager, TextureIdentifiers, TextureManager};
mod score;
use score::Score;
mod particle_manager;
use particle_manager::ParticleManager;
mod util;
use util::are_colors_equal;
mod player;
use player::Player;
mod menu;
use menu::{Button, ButtonType, Menu};

fn generate_platforms(platforms: &mut Vec<Platform>, upper_bound: i32) -> i32 {
    *platforms = vec![Platform::new(
        RectangleShape::default(),
        PlatformType::Static,
        0.,
    )];
    let mut ypos = -300.;

    let mut number_of_plats = 0;
    for i in 0..upper_bound {
        if i != upper_bound - 1 {
            let should_split = rand::thread_rng().gen_range(0i32..6);
            // 2 in 5 chance to split
            if should_split == 4 || should_split == 3 {
                // it's a split
                let split_rand = rand::thread_rng().gen_range(1i32..5);
                let ysize = rand::thread_rng().gen_range(25i32..125) as f32;

                for i in 0..split_rand {
                    let mut new_shape = RectangleShape::default();

                    new_shape.set_size(Vector2f::new(1280. / split_rand as f32, ysize));
                    new_shape.set_fill_color(match rand::thread_rng().gen_range(0i32..4) {
                        0 => Color::RED,
                        1 => Color::GREEN,
                        2 => Color::BLUE,
                        _ => {
                            if split_rand > 1 {
                                Color::WHITE
                            } else {
                                Color::RED
                            }
                        }
                    });

                    let rand_pos = Vector2f::new(1280. / split_rand as f32 * i as f32, ypos);
                    new_shape.set_position(rand_pos);

                    platforms.push(Platform::new(new_shape, PlatformType::Static, 0.));
                    number_of_plats += 1;
                }
            } else {
                // it's not a split
                let ysize = rand::thread_rng().gen_range(25i32..150) as f32;
                let move_speed = rand::thread_rng().gen_range(100i32..500) as f32;

                let mut new_shape = RectangleShape::default();

                new_shape.set_size(Vector2f::new(
                    rand::thread_rng().gen_range(150i32..750) as f32,
                    ysize,
                ));
                new_shape.set_fill_color(match rand::thread_rng().gen_range(0i32..4) {
                    0 => Color::RED,
                    1 => Color::GREEN,
                    2 => Color::BLUE,
                    _ => Color::WHITE,
                });

                let rand_pos = Vector2f::new(
                    rand::thread_rng().gen_range(0i32..1000) as f32,
                    ypos + rand::thread_rng().gen_range(-50i32..50) as f32,
                );
                new_shape.set_position(rand_pos);

                // 50% chance for a moving platform
                let move_roll = rand::thread_rng().gen_range(1i32..3);
                if move_roll == 2 {
                    platforms.push(Platform::new(new_shape, PlatformType::Moving, move_speed));
                } else {
                    platforms.push(Platform::new(new_shape, PlatformType::Static, move_speed));
                }
                number_of_plats += 1;
            }
        } else if i == upper_bound - 1 {
            // create plat that begins next level
            let mut new_shape = RectangleShape::default();
            new_shape.set_size(Vector2f::new(1280., 25.));
            new_shape.set_position(Vector2f::new(
                0.,
                platforms[(number_of_plats) as usize].shape.position().y - 500.,
            ));
            new_shape.set_fill_color(Color::MAGENTA);

            platforms.push(Platform::new(new_shape, PlatformType::Static, 0.));
            number_of_plats += 1;
        }
        ypos -= 200.;
    }
    number_of_plats
}

enum CycleDirection {
    Left,
    Right,
}

fn cycle_colors(player: &mut RectangleShape, direction: CycleDirection) {
    let current_color = player.fill_color();
    let color_values = (
        current_color.red(),
        current_color.green(),
        current_color.blue(),
    );
    let new_color = match direction {
        CycleDirection::Left => match color_values {
            (255, 0, 0) => (0, 0, 255),
            (0, 255, 0) => (255, 0, 0),
            (0, 0, 255) => (0, 255, 0),
            _ => panic!("bad color values recieved in cycle_colors()"),
        },

        CycleDirection::Right => match color_values {
            (255, 0, 0) => (0, 255, 0),
            (0, 255, 0) => (0, 0, 255),
            (0, 0, 255) => (255, 0, 0),
            _ => panic!("bad color values recieved in cycle_colors()"),
        },
    };
    player.set_fill_color(Color::rgb(new_color.0, new_color.1, new_color.2));
}

fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(
        (1280, 720),
        "Confusing Platforms",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);

    let mut font_manager = FontManager::new();
    font_manager.load(FontIdentifiers::Arial, "res/arial.ttf");
    font_manager.load(FontIdentifiers::Joystix, "res/joystix-mono.ttf");

    let mut score = Score::new();
    score
        .text
        .set_font(font_manager.get(FontIdentifiers::Arial));

    let mut game_over_text = Text::default();
    game_over_text.set_font(font_manager.get(FontIdentifiers::Arial));
    game_over_text.set_position(Vector2f::new(1280. / 2. - 175., 250.));
    game_over_text.set_fill_color(Color::WHITE);
    game_over_text.set_character_size(60);
    game_over_text.set_string("GAME OVER!");

    let mut platforms = vec![Platform::new(
        RectangleShape::default(),
        PlatformType::Static,
        0.,
    )];

    const UPPER_BOUND: i32 = 30; //exclusive

    let mut number_of_plats = generate_platforms(&mut platforms, UPPER_BOUND);

    let mut texture_manager = TextureManager::new();
    texture_manager.load(TextureIdentifiers::Nebula, "res/nebula.png");
    texture_manager.load(TextureIdentifiers::Rocket, "res/rocket_small.png");

    let mut player = Player::new();
    player
        .shape
        .set_texture(texture_manager.get(TextureIdentifiers::Rocket), true);

    let mut bg_sprites = vec![
        Sprite::with_texture(texture_manager.get(TextureIdentifiers::Nebula)),
        Sprite::with_texture(texture_manager.get(TextureIdentifiers::Nebula)),
    ];

    bg_sprites[0].set_position(Vector2f::new(0., -720.));
    bg_sprites[1].set_position(Vector2f::new(0., 0.));

    let mut speed_bump = 0.5;

    let mut state_stack = StateStack::new();
    state_stack.push(StateType::Playing);

    let mut particle_manager = ParticleManager::new();

    // delta time
    let mut clock = Clock::start();

    // view
    let mut view = View::new(
        Vector2f::new(1280. / 2., 720. / 2.),
        Vector2f::new(1280., 720.),
    );
    window.set_view(&view);

    // menu
    let mut menu = Menu {
        buttons: vec![
            Button::new(
                font_manager.get(FontIdentifiers::Joystix),
                ButtonType::Resume,
                &Vector2f::new(150., 180.),
            ),
            Button::new(
                font_manager.get(FontIdentifiers::Joystix),
                ButtonType::Quit,
                &Vector2f::new(150., 180. + 80.),
            ),
        ],
    };

    while window.is_open() {
        {
            //___________________ EVENTS_BEGIN ______________//
            while let Some(event) = window.poll_event() {
                match *state_stack.top().unwrap() {
                    StateType::Playing => {
                        match event {
                            Event::Closed => window.close(),
                            Event::MouseMoved { x, .. } => {
                                player
                                    .shape
                                    .set_position(Vector2f::new(x as f32, 720. - 200.));
                            }
                            Event::MouseButtonReleased { button, .. } => match button {
                                mouse::Button::LEFT => {
                                    cycle_colors(&mut player.shape, CycleDirection::Left)
                                }
                                mouse::Button::RIGHT => {
                                    cycle_colors(&mut player.shape, CycleDirection::Right)
                                }
                                _ => {}
                            },
                            Event::KeyReleased { code, .. } => {
                                if let Key::ESCAPE = code {
                                    state_stack.push(StateType::Menu);
                                    println!("{:?}", state_stack);
                                }
                                if let Key::SPACE = code {
                                    player.is_dashing = true;
                                    player.dash_clock.restart();
                                }
                            }
                            _ => { /* do nothing */ }
                        }
                    }
                    StateType::Menu => {
                        match event {
                            Event::KeyReleased { code, .. } => {
                                if code == Key::ESCAPE {
                                    state_stack.pop();
                                    println!("{:?}", state_stack);
                                }
                            }
                            Event::MouseMoved { x, y, .. } => {
                                for button in &mut menu.buttons {
                                    let x = x as f32;
                                    let y = y as f32;
                                    if x > button.text.position().x
                                        && x < (button.text.position().x
                                            + button.text.local_bounds().width)
                                        && y > button.text.position().y
                                        && y < (button.text.position().y
                                            + button.text.local_bounds().height * 2.)
                                    {
                                        // <- *2. because Text bounding box is broken - SFML bug?
                                        button.text.set_fill_color(Color::GREEN);
                                    } else {
                                        button.text.set_fill_color(Color::WHITE);
                                    }
                                }
                            }
                            Event::MouseButtonReleased { button, .. } => {
                                if button == mouse::Button::LEFT {
                                    for button in &menu.buttons {
                                        if are_colors_equal(button.text.fill_color(), Color::GREEN)
                                        {
                                            match button.button_type {
                                                ButtonType::Quit => {
                                                    window.close();
                                                }
                                                ButtonType::Resume => {
                                                    state_stack.pop();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    StateType::GameOver => {
                        match event {
                            Event::Closed => {
                                window.close();
                            }
                            Event::KeyReleased { code, .. } => {
                                if code == Key::R {
                                    //reset the game
                                    state_stack.pop();
                                    score.reset();
                                    number_of_plats =
                                        generate_platforms(&mut platforms, UPPER_BOUND);
                                    speed_bump = 0.5;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            //___________________ EVENTS_END ______________//
        }

        let time = clock.restart();
        match *state_stack.top().unwrap() {
            StateType::Playing => {
                {
                    //___________________ UPDATE_BEGIN ______________//
                    let dt = time.as_seconds();

                    const DASH_SPEED: f32 = 200.;
                    const PLAYER_SPEED: f32 = 200.;
                    let total_speed = if player.is_dashing {
                        PLAYER_SPEED + DASH_SPEED
                    } else {
                        PLAYER_SPEED
                    };

                    // move background
                    for bg in &mut bg_sprites {
                        bg.move_(Vector2f::new(0., (total_speed / 4. + speed_bump) * dt));
                        if bg.position().y >= 720. {
                            bg.move_(Vector2f::new(0., -720. * 2.))
                        }
                    }

                    // reset view
                    view.set_center(Vector2f::new(1280. / 2., 720. / 2.));

                    let mut switch_level = false;

                    // dash expired
                    if player.dash_clock.elapsed_time().as_seconds() >= 0.5 {
                        player.is_dashing = false;
                    }

                    for (i, plat) in platforms.iter_mut().enumerate() {
                        if player
                            .shape
                            .global_bounds()
                            .intersection(&plat.shape.global_bounds())
                            != None
                            && !are_colors_equal(player.shape.fill_color(), plat.shape.fill_color())
                        {
                            if i == (number_of_plats) as usize {
                                switch_level = true;
                            } else {
                                // game over
                                state_stack.push(StateType::GameOver);
                                let score_rect = score.text.local_bounds();
                                score.text.set_origin(Vector2f::new(
                                    score_rect.left + score_rect.width / 2.,
                                    score_rect.top + score_rect.height / 2.,
                                ));
                                score.text.set_character_size(60);
                                score.text.set_position(Vector2f::new(1280. / 2., 350.));
                                score.text.set_fill_color(match score.number {
                                    0..=500 => Color::RED,
                                    501..=1000 => Color::YELLOW,
                                    _ => Color::GREEN,
                                });
                                // reset particles
                                particle_manager.reset();
                            }
                        } else if player
                            .shape
                            .global_bounds()
                            .intersection(&plat.shape.global_bounds())
                            != None
                            && are_colors_equal(player.shape.fill_color(), plat.shape.fill_color())
                        {
                            // player is successfully passing through a platform
                            score.number += (1. * (speed_bump + 1.) * (dt + 1.)) as u32;
                            score.text.set_string(&score.number.to_string());
                            particle_manager.set_position(&player.shape.position());
                            particle_manager.spawn_random_particle(&player.shape.fill_color());

                            // screen shake
                            // make it shake harder when player is dashing
                            let shake_bound: i32 = if player.is_dashing { 6 } else { 2 };
                            let x_offset =
                                rand::thread_rng().gen_range(-shake_bound..shake_bound) as f32;
                            let y_offset =
                                rand::thread_rng().gen_range(-shake_bound..shake_bound) as f32;
                            view.move_((x_offset, y_offset));
                        }

                        // move all platforms downwards
                        plat.shape.move_((0., (total_speed + speed_bump) * dt));

                        let thrust_particle_spawn_time = if player.is_dashing { 0.07 } else { 0.1 };

                        if particle_manager.clock.elapsed_time().as_seconds()
                            >= thrust_particle_spawn_time
                        {
                            particle_manager.set_position(&player.shape.position());
                            particle_manager.spawn_directed_particle(
                                Color::YELLOW,
                                &Vector2f::new(0., 400.),
                                &player.is_dashing,
                            );
                            particle_manager.spawn_directed_particle(
                                Color::YELLOW,
                                &Vector2f::new(-50., 400.),
                                &player.is_dashing,
                            );
                            particle_manager.spawn_directed_particle(
                                Color::YELLOW,
                                &Vector2f::new(50., 400.),
                                &player.is_dashing,
                            );
                            particle_manager.clock.restart();
                        }

                        // check for particle collision with other platforms and mark them for explosion
                        for part in &mut particle_manager.particles {
                            if part
                                .shape
                                .global_bounds()
                                .intersection(&plat.shape.global_bounds())
                                != None
                                && !are_colors_equal(
                                    part.shape.fill_color(),
                                    plat.shape.fill_color(),
                                )
                            {
                                // make sure we don't explode the thruster particles
                                // TODO: perhaps use an enum instead of checking for the color
                                if !are_colors_equal(part.shape.fill_color(), Color::YELLOW) {
                                    part.mark_for_explosion = true;
                                }
                            }
                        }
                    }

                    let speed_bump_dt = speed_bump * dt;
                    for plat in platforms.iter_mut() {
                        plat.move_platform(&speed_bump_dt);
                    }

                    // update particles
                    particle_manager.update(dt, (total_speed + speed_bump) * dt);

                    if switch_level {
                        speed_bump += 0.5;
                        number_of_plats = generate_platforms(&mut platforms, UPPER_BOUND);
                    }
                    //___________________ UPDATE_END ________________//
                }

                {
                    //___________________ RENDER_BEGIN  _____________//
                    // Set view
                    window.set_view(&view);
                    // Clear the window
                    window.clear(Color::BLACK);

                    // Draw bg
                    for bg in &bg_sprites {
                        window.draw(bg);
                    }

                    // Draw the platforms
                    for plat in &platforms {
                        window.draw(&plat.shape);
                    }

                    // Draw particles
                    for p in particle_manager.particles.iter() {
                        window.draw(&p.shape);
                    }

                    // Draw player
                    window.draw(&player.shape);

                    // Draw level text
                    window.draw(&score.text);
                    //____________________ RENDER_END _____________//
                }
            }
            StateType::Menu => {
                // update
                { /* don't update anything for now */ }
                // render
                {
                    window.clear(Color::BLACK);

                    for button in &menu.buttons {
                        window.draw(&button.text);
                    }
                }
            }
            StateType::GameOver => {
                // update
                { /* don't update anything for now */ }
                // render
                {
                    window.clear(Color::BLACK);
                    window.draw(&game_over_text);
                    window.draw(&score.text);
                }
            }
        }
        window.display();
    }
}
