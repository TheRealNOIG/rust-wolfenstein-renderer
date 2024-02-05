use mini_test::*;
use minifb::{Key, Window, WindowOptions};
use std::{
    cmp::min,
    f32::consts::PI,
    time::{Duration, Instant},
};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const MAP_WIDTH: usize = 32;
const MAP_HEIGHT: usize = 32;
const FOV: f32 = PI / 3.0;

const MAP: [u8; MAP_WIDTH * MAP_WIDTH] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

#[derive(Debug)]
struct Player {
    rotation: f32,
    pos_x: f32,
    pos_y: f32,
}
impl Player {
    fn new(x: f32, y: f32, rotation: f32) -> Player {
        Player {
            rotation,
            pos_x: x,
            pos_y: y,
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut last_fps_time = Instant::now();
    let mut frame_count = 0;

    let mut player = Player::new(4.0, 4.0, 0.0);

    let wall_color = rgba::new(0, 115, 0, 255);
    let ceiling_color = rgba::new(0, 0, 155, 255);
    let floor_color = rgba::new(155, 0, 0, 255);

    let (reader, image) = load_image("assets/box.png");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        move_player(&mut player, &window);

        for x in 0..WIDTH {
            let ray_angle = (player.rotation - FOV / 2.0) + (x as f32) * (FOV / WIDTH as f32);
            let (ray, uv) = fast_raycast(
                player.pos_x,
                player.pos_y,
                ray_angle,
                MAP_WIDTH,
                MAP_HEIGHT,
                &MAP,
            );
            let testx = uv * reader.info().width as f32;
            let height = ((HEIGHT as f32) / ray * 2.00) as usize;

            let ceiling_color_u32 = ceiling_color.convert_to_u32();
            let floor_color_u32 = floor_color.convert_to_u32();

            let wall_start = HEIGHT.saturating_sub(height) / 2;
            let wall_end = wall_start + height;

            for y in 0..wall_start {
                buffer[y * WIDTH + x] = ceiling_color_u32;
            }
            for y in wall_start..min(wall_end, HEIGHT) {
                let mut wall_y = y;
                if height > HEIGHT {
                    wall_y = wall_y + (height - HEIGHT) / 2;
                }
                let testy = ((wall_y as f32 - wall_start as f32) / height as f32)
                    * reader.info().height as f32;
                buffer[y * WIDTH + x] =
                    image[testy as usize * reader.info().width as usize + testx as usize];
            }
            for y in wall_end..HEIGHT {
                buffer[y * WIDTH + x] = floor_color_u32;
            }
        }

        for x in 0..reader.info().width as usize {
            for y in 0..reader.info().height as usize {
                buffer[x + y * WIDTH] = image[x + y * reader.info().width as usize];
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        frame_count += 1;
        let elapsed = last_fps_time.elapsed();
        if elapsed >= Duration::from_secs(1) {
            let fps = frame_count;
            window.set_title(&format!("FPS: {}", fps));
            frame_count = 0;
            last_fps_time = Instant::now();
        }
    }
}

fn move_player(player: &mut Player, window: &Window) {
    if window.is_key_down(Key::F) {
        player.pos_x += player.rotation.cos() * 0.1;
        player.pos_y += player.rotation.sin() * 0.1;
    }
    if window.is_key_down(Key::S) {
        player.pos_x -= player.rotation.cos() * 0.1;
        player.pos_y -= player.rotation.sin() * 0.1;
    }
    if window.is_key_down(Key::R) {
        player.pos_x += (player.rotation - PI / 2.0).cos() * 0.1;
        player.pos_y += (player.rotation - PI / 2.0).sin() * 0.1;
    }
    if window.is_key_down(Key::T) {
        player.pos_x += (player.rotation + PI / 2.0).cos() * 0.1;
        player.pos_y += (player.rotation + PI / 2.0).sin() * 0.1;
    }
    if window.is_key_down(Key::P) {
        player.rotation += 0.01;
        if player.rotation > 2.0 * PI {
            player.rotation -= 2.0 * PI;
        }
    }
    if window.is_key_down(Key::W) {
        player.rotation -= 0.01;
        if player.rotation < 0.0 {
            player.rotation += 2.0 * PI;
        }
    }
}

