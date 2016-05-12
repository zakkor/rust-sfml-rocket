extern crate sfml;
extern crate rand;

use rand::Rng;

use sfml::system::Vector2f;
use sfml::window::*;
use sfml::graphics::*;

fn generate_platforms(platforms: &mut Vec<RectangleShape>) {
    let mut ypos = 0.;
    for _ in 0..26 {
        let mut new_plat = RectangleShape::new().unwrap();
        let xsize = rand::thread_rng().gen_range(100, 1281) as f32;
        let ysize = rand::thread_rng().gen_range(25, 125) as f32;
        new_plat.set_size(&Vector2f::new(xsize, ysize));

        let xpos = rand::thread_rng().gen_range(-500, 500) as f32;
        let rand_pos = Vector2f::new(xpos, ypos + rand::thread_rng().gen_range(-50, 50) as f32);

        new_plat.set_position(&rand_pos);
        new_plat.set_fill_color( &match rand::thread_rng().gen_range(0, 4) {
            0 => Color::red(),
            1 => Color::green(),
            2 => Color::blue(),
            _ => Color::white()
        } );

        platforms.push(new_plat);

        ypos -= 200.;
    }
}

fn next_level(platforms: &mut Vec<RectangleShape>, level_count: u8) -> u8 {
    generate_platforms(platforms);
    level_count + 1
}


fn update(platforms: &mut Vec<RectangleShape>, player: &RectangleShape, score: &mut Box<i32>, score_text: &mut Text) -> bool {
    let mut game_over = false;
    for plat in platforms {
        if player.get_global_bounds().intersects(&plat.get_global_bounds()) != None
            && (player.get_fill_color().0.red != plat.get_fill_color().0.red ||
                player.get_fill_color().0.green != plat.get_fill_color().0.green ||
                player.get_fill_color().0.blue != plat.get_fill_color().0.blue) {
                game_over = true;
                score_text.set_string("GAME OVER");
                score_text.set_position(&Vector2f::new(1280. / 2. - 63., 25.));
            }
        else if player.get_global_bounds().intersects(&plat.get_global_bounds()) != None
            && (player.get_fill_color().0.red == plat.get_fill_color().0.red ||
                player.get_fill_color().0.green == plat.get_fill_color().0.green ||
                player.get_fill_color().0.blue == plat.get_fill_color().0.blue) {
                **score = **score + 1;
                score_text.set_string(&score.to_string());
            }
        plat.move2f(0., 3.);
    }
    game_over
}

fn cycle_colors_left(player: &mut RectangleShape) {
    let current_color: Color = player.get_fill_color();
    let new_color =
        match (current_color.0.red, current_color.0.green, current_color.0.blue) {
            (255, 0, 0) => (0, 0, 255),
            (0, 255, 0) => (255, 0, 0),
            (0, 0, 255) => (0, 255, 0),
            (_, _, _) => (0, 0, 0)
         };
    player.set_fill_color(&Color::new_rgb(new_color.0, new_color.1, new_color.2));
}

fn cycle_colors_right(player: &mut RectangleShape) {
    let current_color: Color = player.get_fill_color();
    let new_color =
        match (current_color.0.red, current_color.0.green, current_color.0.blue) {
            (255, 0, 0) => (0, 255, 0),
            (0, 255, 0) => (0, 0, 255),
            (0, 0, 255) => (255, 0, 0),
            (_, _, _) => (0, 0, 0)
        };
    player.set_fill_color(&Color::new_rgb(new_color.0, new_color.1, new_color.2));
}

fn handle_events(window: &mut RenderWindow, player: &mut RectangleShape, game_over: &bool) {
    // Handle events
    for event in window.events() {
        match event {
            event::Closed => window.close(),
            event::MouseMoved { x, .. } => if !game_over {
                player.set_position(&Vector2f::new(x as f32, 720. - 200.));
            },
            event::MouseButtonReleased { button, .. } => {
                match button {
                    MouseButton::Left => cycle_colors_left(player),
                    MouseButton::Right => cycle_colors_right(player),
                    _ => { }
                }
            },
            event::KeyReleased { code, .. } => { println!("{:?}", code) }
            _             => {/* do nothing */}
        }
    }
}

fn render(window: &mut RenderWindow, player: &RectangleShape, platforms: &Vec<RectangleShape>, score_text: &Text) {
    // Clear the window
    window.clear(&Color::black());

    // Draw the platforms
    for plat in platforms {
        window.draw(plat);
    }

    // Draw player
    window.draw(player);

    // Draw level text
    window.draw(score_text);

    // Display things on screen
    window.display();
}


fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(VideoMode::new_init(1280, 720, 32),
                                             "SFML Example",
                                             window_style::CLOSE,
                                       &ContextSettings::default()).unwrap();
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);

    let font = Font::new_from_file("res/arial.ttf").unwrap();
    let mut score = Box::new(0);
    let mut score_text = Text::new().unwrap();
    score_text.set_font(&font);
    score_text.set_position(&Vector2f::new(1280. / 2., 25.));
    score_text.set_color(&Color::white());
    score_text.set_string(&score.to_string());

    let mut platforms = vec![RectangleShape::new().unwrap()];

    let mut level_count: u8 = 0;
    level_count = next_level(&mut platforms, level_count);

    let mut player = RectangleShape::new().unwrap();
    player.set_size(&Vector2f::new(25., 25.));
    player.set_fill_color(&Color::red());
    player.set_position(&Vector2f::new(1280. / 2., 720. - 200.));
    player.set_outline_thickness(3.);
    player.set_outline_color(&Color::white());

    let mut game_over = false;

    while window.is_open() {
        handle_events(&mut window, &mut player, &game_over);

        render(&mut window, &player, &platforms, &score_text);

        // Update
        if !game_over {
            game_over = update(&mut platforms, &player, &mut score, &mut score_text);
        }
    }
}
