extern crate sfml;
extern crate rand;

use rand::Rng;

use sfml::graphics::*; use sfml::system::*; use sfml::window::*;

mod platform; use platform::*;
mod state_stack; use state_stack::*;
mod resource_manager; use resource_manager::*;
mod score; use score::Score;
mod particle_manager; use particle_manager::*;
mod util; use util::are_colors_equal;
mod player; use player::*;

fn generate_platforms(platforms: &mut Vec<Platform>, upper_bound: i32) -> i32 {
    *platforms = vec![Platform::new(RectangleShape::new().unwrap(), PlatformType::Static, 0.)];
    let mut ypos = -300.;

    let mut number_of_plats = 0;
    for i in 0..upper_bound {
        if i != upper_bound - 1 {
            let should_split = rand::thread_rng().gen_range(0, 6);
            // 2 in 5 chance to split
            if should_split == 4 || should_split == 3 {
                // it's a split
                let split_rand = rand::thread_rng().gen_range(1, 5);
                let ysize = rand::thread_rng().gen_range(25, 125) as f32;

                for i in 0..split_rand {
                    let mut new_shape = RectangleShape::new().unwrap();

                    new_shape.set_size(&Vector2f::new(1280. / split_rand as f32, ysize));
                    new_shape.set_fill_color(&match rand::thread_rng().gen_range(0, 4) {
                        0 => Color::red(),
                        1 => Color::green(),
                        2 => Color::blue(),
                        _ => if split_rand > 1 { Color::white() } else { Color::red() },
                    });

                    let rand_pos = Vector2f::new(1280. / split_rand as f32 * i as f32, ypos);
                    new_shape.set_position(&rand_pos);

                    platforms.push(Platform::new(new_shape, PlatformType::Static, 0.));
                    number_of_plats += 1;
                }
            } else {
                // it's not a split
                let ysize = rand::thread_rng().gen_range(25, 150) as f32;
                let move_speed = rand::thread_rng().gen_range(100, 500) as f32;

                let mut new_shape = RectangleShape::new().unwrap();

                new_shape.set_size(&Vector2f::new(rand::thread_rng().gen_range(150, 750) as f32,
                                                  ysize));
                new_shape.set_fill_color(&match rand::thread_rng().gen_range(0, 4) {
                    0 => Color::red(),
                    1 => Color::green(),
                    2 => Color::blue(),
                    _ => Color::white(),
                });

                let rand_pos = Vector2f::new(rand::thread_rng().gen_range(0, 1000) as f32,
                                             ypos + rand::thread_rng().gen_range(-50, 50) as f32);
                new_shape.set_position(&rand_pos);

                // 50% chance for a moving platform
                let move_roll = rand::thread_rng().gen_range(1, 3);
                if move_roll == 2 {
                    platforms.push(Platform::new(new_shape, PlatformType::Moving, move_speed));
                }
                else {
                    platforms.push(Platform::new(new_shape, PlatformType::Static, move_speed));
                }
                number_of_plats += 1;
            }
        } else if i == upper_bound - 1 {
            // create plat that begins next level
            let mut new_shape = RectangleShape::new().unwrap();
            new_shape.set_size(&Vector2f::new(1280., 25.));
            new_shape.set_position(&Vector2f::new(0., platforms[(number_of_plats) as usize].shape.get_position().y - 500.));
            new_shape.set_fill_color(&Color::magenta());

            platforms.push(Platform::new(new_shape, PlatformType::Static, 0.));
            number_of_plats += 1;
        }
        ypos -= 200.;
    }
    number_of_plats
}

enum CycleDirection {
    Left,
    Right
}

fn cycle_colors(player: &mut RectangleShape, direction: CycleDirection) {
    let current_color = player.get_fill_color();
    let color_values = (current_color.0.red, current_color.0.green, current_color.0.blue);
    let new_color =
        match direction {
            CycleDirection::Left =>
                match color_values {
                    (255, 0, 0) => (0, 0, 255),
                    (0, 255, 0) => (255, 0, 0),
                    (0, 0, 255) => (0, 255, 0),
                    _ => panic!("bad color values recieved in cycle_colors()"),
                },

            CycleDirection::Right =>
                match color_values {
                    (255, 0, 0) => (0, 255, 0),
                    (0, 255, 0) => (0, 0, 255),
                    (0, 0, 255) => (255, 0, 0),
                    _ => panic!("bad color values recieved in cycle_colors()"),
                }
        };
    player.set_fill_color(&Color::new_rgb(new_color.0, new_color.1, new_color.2));
}

fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(VideoMode::new_init(1280, 720, 32),
                                       "Confusing Platforms",
                                       window_style::CLOSE,
                                       &ContextSettings::default())
        .unwrap();
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);

    let mut font_manager = FontManager::new();
    font_manager.load(FontIdentifiers::Arial, "res/arial.ttf");

    let mut score = Score::new();
    score.text.set_font(font_manager.get(FontIdentifiers::Arial));

    let mut game_over_text = Text::new().unwrap();
    game_over_text.set_font(font_manager.get(FontIdentifiers::Arial));
    game_over_text.set_position(&Vector2f::new(1280. / 2. - 175., 250.));
    game_over_text.set_color(&Color::white());
    game_over_text.set_character_size(60);
    game_over_text.set_string("GAME OVER!");


    let mut platforms = vec![Platform::new(RectangleShape::new().unwrap(), PlatformType::Static, 0.)];

    const UPPER_BOUND: i32 = 30; //exclusive

    let mut number_of_plats = generate_platforms(&mut platforms, UPPER_BOUND);

    let mut texture_manager = TextureManager::new();
    texture_manager.load(TextureIdentifiers::Nebula, "res/nebula.png");
    texture_manager.load(TextureIdentifiers::Rocket, "res/rocket_small.png");

    let mut player = Player::new();
    player.shape.set_texture(texture_manager.get(TextureIdentifiers::Rocket), true);

    let mut bg_sprites = vec![Sprite::new_with_texture(texture_manager.get(TextureIdentifiers::Nebula)).unwrap(),
                              Sprite::new_with_texture(texture_manager.get(TextureIdentifiers::Nebula)).unwrap()];

    bg_sprites[0].set_position(&Vector2f::new(0., -720.));
    bg_sprites[1].set_position(&Vector2f::new(0., 0.));

    let mut speed_bump = 0.5;

    let mut state_stack = StateStack::new();
    state_stack.push(StateType::Playing);

    let mut particle_manager = ParticleManager::new();

    // delta time
    let mut clock = Clock::new();

    // view
    let mut view = View::new_init(&Vector2f::new(1280./2., 720./2.), &Vector2f::new(1280., 720.)).unwrap();
    window.set_view(&view);

    while window.is_open() {
        {
            //___________________ EVENTS_BEGIN ______________//
            for event in window.events() {
                match *state_stack.top().unwrap() {
                    StateType::Playing => {
                        match event {
                            event::Closed => window.close(),
                            event::MouseMoved { x, .. } => {
                                player.shape.set_position(&Vector2f::new(x as f32, 720. - 200.));
                            }
                            event::MouseButtonReleased { button, .. } => {
                                match button {
                                    MouseButton::Left => cycle_colors(&mut player.shape, CycleDirection::Left),
                                    MouseButton::Right => cycle_colors(&mut player.shape, CycleDirection::Right),
                                    _ => {}
                                }
                            }
                            event::KeyReleased { code, .. } => {
                                if let Key::Escape = code {
                                    state_stack.push(StateType::Menu);
                                    println!("{:?}", state_stack);
                                }
                                if let Key::Space = code {
                                    player.is_dashing = true;
                                    player.dash_clock.restart();
                                }
                            }
                            _ => { /* do nothing */ }
                        }
                    },
                    StateType::Menu => {
                        if let event::KeyReleased { code, .. } = event {
                            if let Key::Escape = code {
                                state_stack.pop();
                                println!("{:?}", state_stack);
                            }
                        }
                    },
                    StateType::GameOver => {
                        match event {
                            event::Closed => { window.close(); },
                            event::KeyReleased { code, .. } => {
                                match code {
                                    Key::R => {
                                        //reset the game
                                        state_stack.pop();
                                        score.reset();
                                        number_of_plats = generate_platforms(&mut platforms, UPPER_BOUND);
                                        speed_bump = 0.5;
                                    },
                                    _ => {}
                                }
                            },
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
                    let total_speed = if player.is_dashing { PLAYER_SPEED + DASH_SPEED } else { PLAYER_SPEED };

                    // move background
                    for bg in &mut bg_sprites {
                        bg.move_(&Vector2f::new(0., (total_speed / 4. + speed_bump) * dt ));
                        if bg.get_position().y >= 720. {
                            bg.move_(&Vector2f::new(0., -720. * 2.))
                        }
                    }

                    // reset view
                    view.set_center(&Vector2f::new(1280./2., 720./2.));

                    let mut switch_level = false;

                    // dash expired
                    if player.dash_clock.get_elapsed_time().as_seconds() >= 0.5 {
                        player.is_dashing = false;
                    }

                    for (i, plat) in platforms.iter_mut().enumerate() {
                        if player.shape.get_global_bounds().intersects(&plat.shape.get_global_bounds()) != None &&
                            !are_colors_equal(&player.shape.get_fill_color(), &plat.shape.get_fill_color()) {
                                if i == (number_of_plats) as usize {
                                    switch_level = true;
                                } else {
                                    // game over
                                    state_stack.push(StateType::GameOver);
                                    let score_rect = score.text.get_local_bounds();
                                    score.text.set_origin(&Vector2f::new(score_rect.left + score_rect.width / 2.,
                                                                         score_rect.top + score_rect.height / 2.));
                                    score.text.set_character_size(60);
                                    score.text.set_position(&Vector2f::new(1280. / 2., 350.));
                                    score.text.set_color(
                                        &match score.number {
                                            0...500 => Color::red(),
                                            501...1000 => Color::yellow(),
                                            _ => Color::green()
                                        } );
                                    // reset particles
                                    particle_manager.reset();
                                }

                            }
                        else if player.shape.get_global_bounds().intersects(&plat.shape.get_global_bounds()) != None &&
                            are_colors_equal(&player.shape.get_fill_color(), &plat.shape.get_fill_color()) {
                                // player is successfully passing through a platform
                                score.number += (1. * (speed_bump + 1.) * (dt + 1.)) as u32;
                                score.text.set_string(&score.number.to_string());
                                particle_manager.set_position(&player.shape.get_position());
                                particle_manager.spawn_random_particle(&player.shape.get_fill_color());

                                // screen shake
                                // make it shake harder when player is dashing
                                let shake_bound = if player.is_dashing { 6 } else { 2 };
                                let x_offset = rand::thread_rng().gen_range(-shake_bound, shake_bound) as f32;
                                let y_offset = rand::thread_rng().gen_range(-shake_bound, shake_bound) as f32;
                                view.move2f(x_offset, y_offset);
                            }

                        // move all platforms downwards
                        plat.shape.move2f(0., (total_speed + speed_bump) * dt);

                        let thrust_particle_spawn_time = if player.is_dashing { 0.07 } else { 0.1 };

                        if particle_manager.clock.get_elapsed_time().as_seconds() >= thrust_particle_spawn_time {
                            particle_manager.set_position(&player.shape.get_position());
                            particle_manager.spawn_directed_particle(&Color::yellow(), &Vector2f::new(0., 400.), &player.is_dashing);
                            particle_manager.spawn_directed_particle(&Color::yellow(), &Vector2f::new(-50., 400.), &player.is_dashing);
                            particle_manager.spawn_directed_particle(&Color::yellow(), &Vector2f::new(50., 400.), &player.is_dashing);
                            particle_manager.clock.restart();
                        }

                        // check for particle collision with other platforms and mark them for explosion
                        for part in &mut particle_manager.particles {
                            if part.shape.get_global_bounds().intersects(&plat.shape.get_global_bounds()) != None &&
                                !are_colors_equal(&part.shape.get_fill_color(), &plat.shape.get_fill_color()) {
                                    // make sure we don't explode the thruster particles
                                    // TODO: perhaps use an enum instead of checking for the color
                                    if !are_colors_equal(&part.shape.get_fill_color(), &Color::yellow()) {
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
                    window.clear(&Color::black());

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
            },
            StateType::Menu => {
                {
                    /* don't update anything for now */
                }
                {
                    /* don't draw anything for now */
                }
                window.clear(&Color::blue());
            },
            StateType::GameOver => {
                {
                    /* don't update anything for now */
                }

                {
                    window.clear(&Color::black());
                    window.draw(&game_over_text);
                    window.draw(&score.text);
                }
            }
        }
        window.display();
    }
}
