extern crate sfml;
extern crate rand;

use rand::Rng;

use sfml::system::Vector2f;
use sfml::window::*;
use sfml::graphics::*;

fn generate_platforms(platforms: &mut Vec<RectangleShape>, upper_bound: i32) -> i32 {
    *platforms = vec![RectangleShape::new().unwrap()];
    let mut ypos = -300.;

    let mut number_of_plats = 0;
    for i in 0..upper_bound {
        if i != upper_bound - 1 {
            let should_split = rand::thread_rng().gen_range(0, 6);
            if should_split == 4 || should_split == 3 {
                // 2 in 5 chance to split
                let split_rand = rand::thread_rng().gen_range(1, 5);

                // let xsize = rand::thread_rng().gen_range(100, 1281) as f32;
                let ysize = rand::thread_rng().gen_range(25, 125) as f32;

                for i in 0..split_rand {
                    // let xpos_and_size = rand::thread_rng().gen_range(-500, 500) as f32;
                    let mut new_plat = RectangleShape::new().unwrap();

                    new_plat.set_size(&Vector2f::new(1280. / split_rand as f32, ysize));
                    new_plat.set_fill_color(&match rand::thread_rng().gen_range(0, 4) {
                        0 => Color::red(),
                        1 => Color::green(),
                        2 => Color::blue(),
                        _ => if split_rand > 1 { Color::white() } else { Color::red() },
                    });

                    let rand_pos = Vector2f::new(1280. / split_rand as f32 * i as f32, ypos);// + rand::thread_rng().gen_range(-50, 50) as f32);
                    new_plat.set_position(&rand_pos);
                    platforms.push(new_plat);
                    number_of_plats += 1;
                }
            } else {
                let ysize = rand::thread_rng().gen_range(25, 150) as f32;

                let mut new_plat = RectangleShape::new().unwrap();

                new_plat.set_size(&Vector2f::new(rand::thread_rng().gen_range(150, 750) as f32,
                                                 ysize));
                new_plat.set_fill_color(&match rand::thread_rng().gen_range(0, 4) {
                    0 => Color::red(),
                    1 => Color::green(),
                    2 => Color::blue(),
                    _ => Color::white(),
                });

                let rand_pos = Vector2f::new(rand::thread_rng().gen_range(0, 1000) as f32,
                                             ypos + rand::thread_rng().gen_range(-50, 50) as f32);
                new_plat.set_position(&rand_pos);
                platforms.push(new_plat);
                number_of_plats += 1;
            }
        } else if i == upper_bound - 1 {
            // create plat that begins next level
            let mut new_plat = RectangleShape::new().unwrap();
            new_plat.set_size(&Vector2f::new(1280., 25.));
            new_plat.set_position(&Vector2f::new(0.,
                                                 platforms[(number_of_plats) as usize]
                                                     .get_position()
                                                     .y -
                                                 500.));
            new_plat.set_fill_color(&Color::magenta());
            platforms.push(new_plat);
            number_of_plats += 1;
        }


        // platforms.push(new_plat);

        ypos -= 200.;
    }
    number_of_plats
}

fn next_level(platforms: &mut Vec<RectangleShape>, upper_bound: i32) -> i32 {
    generate_platforms(platforms, upper_bound)
}


fn update(platforms: &mut Vec<RectangleShape>,
          player: &RectangleShape,
          score: &mut Box<i32>,
          score_text: &mut Text,
          bg_sprites: &mut Vec<Sprite>,
          upper_bound: i32,
          number_of_plats: &mut i32,
          speed_bump: &mut f32)
          -> bool {
    let mut game_over = false;

    for bg in bg_sprites {
        bg.move_(&Vector2f::new(0., 1. + *speed_bump));
        if bg.get_position().y >= 720. {
            // let bg_ypos = bg.get_position().y;
            // bg.set_position((&Vector2f::new(0., -bg_ypos - (720. - bg_ypos - (2. * (*speed_bump))))))
            bg.move_(&Vector2f::new(0., -720. * 2.))
        }
    }


    let mut switch_level = false;

    for (i, plat) in platforms.iter_mut().enumerate() {
        if player.get_global_bounds().intersects(&plat.get_global_bounds()) != None &&
           (player.get_fill_color().0.red != plat.get_fill_color().0.red ||
            player.get_fill_color().0.green != plat.get_fill_color().0.green ||
            player.get_fill_color().0.blue != plat.get_fill_color().0.blue) {
            if i == (*number_of_plats) as usize {
                switch_level = true;
            } else {
                game_over = true;
                //                    score_text.set_string("GAME OVER");
                //                    score_text.set_position(&Vector2f::new(1280. / 2. - 63., 25.));
            }

        } else if player.get_global_bounds().intersects(&plat.get_global_bounds()) != None &&
           (player.get_fill_color().0.red == plat.get_fill_color().0.red ||
            player.get_fill_color().0.green == plat.get_fill_color().0.green ||
            player.get_fill_color().0.blue == plat.get_fill_color().0.blue) {
            **score += (1. * (*speed_bump + 1.)) as i32;
            score_text.set_string(&score.to_string());
        }
        plat.move2f(0., 3. + *speed_bump);
    }

    if switch_level {
        *speed_bump += 0.5;
        // *level_count = next_level(platforms, *level_count, upper_bound);
        *number_of_plats = next_level(platforms, upper_bound);
    }

    game_over
}

fn cycle_colors_left(player: &mut RectangleShape) {
    let current_color: Color = player.get_fill_color();
    let new_color = match (current_color.0.red, current_color.0.green, current_color.0.blue) {
        (255, 0, 0) => (0, 0, 255),
        (0, 255, 0) => (255, 0, 0),
        (0, 0, 255) => (0, 255, 0),
        (_, _, _) => (0, 0, 0),
    };
    player.set_fill_color(&Color::new_rgb(new_color.0, new_color.1, new_color.2));
}

fn cycle_colors_right(player: &mut RectangleShape) {
    let current_color: Color = player.get_fill_color();
    let new_color = match (current_color.0.red, current_color.0.green, current_color.0.blue) {
        (255, 0, 0) => (0, 255, 0),
        (0, 255, 0) => (0, 0, 255),
        (0, 0, 255) => (255, 0, 0),
        (_, _, _) => (0, 0, 0),
    };
    player.set_fill_color(&Color::new_rgb(new_color.0, new_color.1, new_color.2));
}

fn handle_events(window: &mut RenderWindow,
                 player: &mut RectangleShape,
                 game_over: &mut bool,
                 score: &mut i32,
                 score_text: &mut Text,
                 platforms: &mut Vec<RectangleShape>,
                 upper_bound: i32,
                 number_of_plats: &mut i32,
                 speed_bump: &mut f32) {
    // Handle events
    for event in window.events() {
        match event {
            event::Closed => window.close(),
            event::MouseMoved { x, .. } => {
                if !*game_over {
                    player.set_position(&Vector2f::new(x as f32, 720. - 200.));
                }
            }
            event::MouseButtonReleased { button, .. } => {
                match button {
                    MouseButton::Left => cycle_colors_left(player),
                    MouseButton::Right => cycle_colors_right(player),
                    _ => {}
                }
            }
            event::KeyReleased { code, .. } => {
                match code {
                    Key::Escape => window.close(),
                    Key::R => {
                        *game_over = false;
                        *score = 0;
                        score_text.set_string("0");
                        *number_of_plats = next_level(platforms, upper_bound);
                        *speed_bump = 0.;
                    }
                    _ => {}
                }
            }
            _ => {
                // do nothing
            }
        }
    }
}

fn render(window: &mut RenderWindow,
          player: &RectangleShape,
          platforms: &Vec<RectangleShape>,
          score_text: &Text,
          game_over_text: &Text,
          bg_sprites: &Vec<Sprite>,
          game_over: &bool) {
    // Clear the window
    window.clear(&Color::black());

    // Draw bg
    for bg in bg_sprites {
        window.draw(bg);
    }

    if !game_over {
        // Draw the platforms
        for plat in platforms {
            window.draw(plat);
        }

        // Draw player
        window.draw(player);
    } else {
        window.draw(game_over_text);
    }

    // Draw level text
    window.draw(score_text);

    // Display things on screen
    window.display();
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

    let font = Font::new_from_file("res/arial.ttf").unwrap();

    let mut score = Box::new(0);

    let mut score_text = Text::new().unwrap();
    score_text.set_font(&font);
    score_text.set_position(&Vector2f::new(1280. / 2., 25.));
    score_text.set_color(&Color::white());
    score_text.set_string(&score.to_string());

    let mut game_over_text = Text::new().unwrap();
    game_over_text.set_font(&font);
    game_over_text.set_position(&Vector2f::new(1280. / 2. - 175., 250.));
    game_over_text.set_color(&Color::white());
    game_over_text.set_character_size(60);
    game_over_text.set_string("GAME OVER!");


    let mut platforms = vec![RectangleShape::new().unwrap()];

    let mut level_count: u8 = 0;
    const UPPER_BOUND: i32 = 30; //exclusive
    //level_count =
    let mut number_of_plats = next_level(&mut platforms, UPPER_BOUND);

    let mut player = RectangleShape::new().unwrap();
    player.set_size(&Vector2f::new(25., 25.));
    player.set_fill_color(&Color::red());
    player.set_position(&Vector2f::new(1280. / 2., 720. - 200.));
    player.set_outline_thickness(3.);
    player.set_outline_color(&Color::white());

    let bg_texture = Texture::new_from_file("res/nebula.png").unwrap();

    let mut bg_sprites = vec![Sprite::new_with_texture(&bg_texture).unwrap(),
                              Sprite::new_with_texture(&bg_texture).unwrap()];
    bg_sprites[0].set_position(&Vector2f::new(0., -720.));
    bg_sprites[1].set_position(&Vector2f::new(0., 0.));

    let mut game_over = false;

    let mut speed_bump = 0.;

    while window.is_open() {
        handle_events(&mut window,
                      &mut player,
                      &mut game_over,
                      &mut score,
                      &mut score_text,
                      &mut platforms,
                      UPPER_BOUND,
                      &mut number_of_plats,
                      &mut speed_bump);

        // Update
        if !game_over {
            game_over = update(&mut platforms,
                               &player,
                               &mut score,
                               &mut score_text,
                               &mut bg_sprites,
                               UPPER_BOUND,
                               &mut number_of_plats,
                               &mut speed_bump);
        }

        render(&mut window,
               &player,
               &platforms,
               &score_text,
               &game_over_text,
               &bg_sprites,
               &game_over);
    }
}
