use std::{
    f32::consts::PI,
    time::{Duration, Instant},
};

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const MAP_WIDTH: usize = 32;
const FOV: f32 = PI / 3.0;

const STEP_INCREMENT: f32 = 0.01;
const MAX_DISTANCE: f32 = 64.0;

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

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
struct rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl rgba {
    fn new(r: u8, g: u8, b: u8, a: u8) -> rgba {
        rgba { r, g, b, a }
    }
    #[inline]
    fn convert_to_u32(self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        move_player(&mut player, &window);

        for x in 0..WIDTH {
            let ray_angle = (player.rotation - FOV / 2.0) + (x as f32) * (FOV / WIDTH as f32);
            let ray = slow_raycast(player.pos_x, player.pos_y, ray_angle, &MAP);
            let height = ((HEIGHT as f32) / ray * 2.00) as usize;

            draw_line(
                x,
                height,
                ceiling_color,
                wall_color,
                floor_color,
                &mut buffer,
            );
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

#[allow(dead_code)]
fn draw_pixel(x: usize, y: usize, color: rgba, buffer: &mut [u32]) {
    buffer[y * WIDTH + x] = color.convert_to_u32();
}
#[allow(dead_code)]
fn draw_pixel_u32(x: usize, y: usize, color: u32, buffer: &mut [u32]) {
    buffer[y * WIDTH + x] = color;
}

/// Draws a vertical line on the screen buffer with specified colors for the ceiling, wall, and floor.
///
/// # Parameters:
/// - `x`: The x-coordinate of the line to draw.
/// - `height`: The height of the wall section to draw.
/// - `ceiling_color`: The color of the ceiling section.
/// - `wall_color`: The color of the wall section.
/// - `floor_color`: The color of the floor section.
/// - `buffer`: A mutable reference to the screen buffer where pixels are drawn.
///
/// The function divides the line into three sections: ceiling, wall, and floor,
/// and colors each section accordingly. It automatically adjusts for the screen height
/// and ensures that the drawing does not exceed the screen bounds.
fn draw_line(
    x: usize,
    wall_height: usize,
    ceiling_color: rgba,
    wall_color: rgba,
    floor_color: rgba,
    buffer: &mut [u32],
) {
    let wall_color_u32 = wall_color.convert_to_u32();
    let ceiling_color_u32 = ceiling_color.convert_to_u32();
    let floor_color_u32 = floor_color.convert_to_u32();

    let wall_start = HEIGHT.saturating_sub(wall_height) / 2;
    let wall_end = wall_start + wall_height;

    for y in 0..wall_start {
        buffer[y * WIDTH + x] = ceiling_color_u32;
    }
    for y in wall_start..std::cmp::min(wall_end, HEIGHT) {
        buffer[y * WIDTH + x] = wall_color_u32;
    }
    for y in wall_end..HEIGHT {
        buffer[y * WIDTH + x] = floor_color_u32;
    }
}

/// Performs a raycast in a 2D grid map, returning the distance to the first encountered obstacle.
///
/// # Parameters:
/// - `start_x`: The starting x-coordinate of the ray.
/// - `start_y`: The starting y-coordinate of the ray.
/// - `theta`: The angle of the ray in radians.
/// - `map`: A reference to the 2D grid map, represented as a linear array.
///
/// # Returns:
/// The distance to the first obstacle hit by the ray, or the maximum distance if no obstacle is encountered.
fn slow_raycast(start_x: f32, start_y: f32, theta: f32, map: &[u8; MAP_WIDTH * MAP_WIDTH]) -> f32 {
    let mut distance = 0.0;
    let (mut x, mut y) = (start_x, start_y);
    let (dx, dy) = (theta.cos() * STEP_INCREMENT, theta.sin() * STEP_INCREMENT);

    while distance < MAX_DISTANCE {
        // Calculate the grid position
        let grid_x = x as usize;
        let grid_y = y as usize;

        // Early exit if out of bounds
        if grid_x >= MAP_WIDTH || grid_y >= MAP_WIDTH {
            break;
        }
        // Check for a hit
        if map[grid_x + grid_y * MAP_WIDTH] == 1 {
            return distance;
        }

        // Move the ray forward
        x += dx;
        y += dy;
        distance += STEP_INCREMENT;
    }

    distance
}
