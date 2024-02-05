use core::f32;
use std::{
    cmp::{max, min},
    fs::File,
};

use png::{Decoder, Reader, Transformations};

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub struct rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> rgba {
        rgba { r, g, b, a }
    }
    #[inline]
    pub fn convert_to_u32(self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
    pub fn from_u32(color: u32) -> rgba {
        // Extract each component by shifting and masking
        let a = ((color >> 24) & 0xFF) as u8; // Shift right 24 bits and mask with 0xFF to get alpha
        let r = ((color >> 16) & 0xFF) as u8; // Shift right 16 bits and mask with 0xFF to get red
        let g = ((color >> 8) & 0xFF) as u8; // Shift right 8 bits and mask with 0xFF to get green
        let b = (color & 0xFF) as u8; // Mask with 0xFF to get blue

        rgba::new(r, g, b, a)
    }
}

#[allow(dead_code)]
fn draw_pixel(x: usize, y: usize, color: rgba, buffer_width: usize, buffer: &mut [u32]) {
    buffer[y * buffer_width + x] = color.convert_to_u32();
}
#[allow(dead_code)]
fn draw_pixel_u32(x: usize, y: usize, color: u32, buffer_width: usize, buffer: &mut [u32]) {
    buffer[y * buffer_width + x] = color;
}

/// Draws a vertical line on the screen buffer with specified colors for the ceiling, wall, and floor.
///
/// # Parameters:
/// - `x`: The x-coordinate of the line to draw.
/// - `wall_height`: The height of the wall section to draw.
/// - `ceiling_color`: The color of the ceiling section as an `rgba` struct.
/// - `wall_color`: The color of the wall section as an `rgba` struct.
/// - `floor_color`: The color of the floor section as an `rgba` struct.
/// - `buffer_width`: The width of the screen buffer.
/// - `buffer_height`: The height of the screen buffer.
/// - `buffer`: A mutable reference to the screen buffer (array of `u32` representing pixels).
///
/// The function divides the line into three sections: ceiling, wall, and floor,
/// and colors each section accordingly. It automatically adjusts for the screen height
/// and ensures that the drawing does not exceed the screen bounds.
pub fn draw_line(
    x: usize,
    wall_height: usize,
    ceiling_color: rgba,
    wall_color: rgba,
    floor_color: rgba,
    buffer_width: usize,
    buffer_height: usize,
    buffer: &mut [u32],
) {
    let wall_color_u32 = wall_color.convert_to_u32();
    let ceiling_color_u32 = ceiling_color.convert_to_u32();
    let floor_color_u32 = floor_color.convert_to_u32();

    let wall_start = buffer_height.saturating_sub(wall_height) / 2;
    let wall_end = wall_start + wall_height;

    for y in 0..wall_start {
        buffer[y * buffer_width + x] = ceiling_color_u32;
    }
    for y in wall_start..min(wall_end, buffer_height) {
        buffer[y * buffer_width + x] = wall_color_u32;
    }
    for y in wall_end..buffer_height {
        buffer[y * buffer_width + x] = floor_color_u32;
    }
}

/// Performs a raycast in a 2D grid map, returning the distance to the first encountered obstacle.
///
/// # Parameters:
/// - `start_x`: The starting x-coordinate of the ray.
/// - `start_y`: The starting y-coordinate of the ray.
/// - `theta`: The angle of the ray in radians.
/// - `step_increment`: The increment for each step of the ray.
/// - `max_distance`: The maximum distance the ray can travel.
/// - `map_width`: The width of the 2D grid map.
/// - `map_height`: The height of the 2D grid map.
/// - `map`: A reference to the 2D grid map, represented as a linear array.
///
/// # Returns:
/// The distance to the first obstacle hit by the ray, or the maximum distance if no obstacle is encountered.
pub fn slow_raycast(
    start_x: f32,
    start_y: f32,
    theta: f32,
    step_increment: f32,
    map_width: usize,
    map_height: usize,
    map: &[u8],
) -> f32 {
    let mut distance = 0.0;
    let (mut x, mut y) = (start_x, start_y);
    let (dx, dy) = (theta.cos() * step_increment, theta.sin() * step_increment);

    while distance < max(map_width, map_height) as f32 {
        let grid_x = x as usize;
        let grid_y = y as usize;

        if grid_x >= map_width || grid_y >= map_height {
            break;
        }
        if map[grid_x + grid_y * map_width] == 1 {
            return distance;
        }

        x += dx;
        y += dy;
        distance += step_increment;
    }

    distance
}

/// Performs a fast raycast in a 2D grid map to find the distance to the first obstacle from a given starting point.
///
/// # Arguments
///
/// * `start_x` - The x-coordinate of the starting point of the ray.
/// * `start_y` - The y-coordinate of the starting point of the ray.
/// * `theta` - The angle (in radians) of the ray direction from the positive x-axis.
/// * `map_width` - The width of the map.
/// * `map_height` - The height of the map.
/// - `map`: A reference to the 2D grid map, represented as a linear array.
///
/// # Returns
///
/// The function returns the distance to the first obstacle encountered in the direction of `theta`. If no obstacle is encountered within the map bounds, `f32::MAX` is returned.
pub fn fast_raycast(
    start_x: f32,
    start_y: f32,
    theta: f32,
    map_width: usize,
    map_height: usize,
    map: &[u8],
) -> (f32, f32) {
    let mut map_x = start_x as isize;
    let mut map_y = start_y as isize;

    let dir_x = theta.cos();
    let dir_y = theta.sin();

    let delta_dist_x = (1.0 / dir_x).abs().max(f32::MIN_POSITIVE);
    let delta_dist_y = (1.0 / dir_y).abs().max(f32::MIN_POSITIVE);

    let step_x = if dir_x >= 0.0 { 1 } else { -1 };
    let step_y = if dir_y >= 0.0 { 1 } else { -1 };

    let mut ray_x = match dir_x.signum() as isize {
        1 => (1.0 - start_x.fract()) * delta_dist_x,
        -1 => start_x.fract() * delta_dist_x,
        _ => f32::MAX,
    };
    let mut ray_y = match dir_y.signum() as isize {
        1 => (1.0 - start_y.fract()) * delta_dist_y,
        -1 => start_y.fract() * delta_dist_y,
        _ => f32::MAX,
    };

    let mut side;
    while map_x >= 0 && map_x < map_width as isize && map_y >= 0 && map_y < map_height as isize {
        if ray_x < ray_y {
            ray_x += delta_dist_x;
            map_x += step_x;
            side = 0;
        } else {
            ray_y += delta_dist_y;
            map_y += step_y;
            side = 1;
        }

        if map_x < 0 || map_x >= map_width as isize || map_y < 0 || map_y >= map_height as isize {
            break;
        }

        if map[(map_x as usize) + (map_y as usize) * map_width] == 1 {
            if side == 0 {
                let perp_wall_dis = ray_x - delta_dist_x;
                return (perp_wall_dis, (start_y + perp_wall_dis * dir_y).fract());
            } else {
                let perp_wall_dis = ray_y - delta_dist_y;
                return (perp_wall_dis, (start_x + perp_wall_dis * dir_x).fract());
            }
        }
    }

    (f32::MAX, f32::MAX)
}

// https://github.com/emoon/rust_minifb/blob/ef07f55834d711a88676f011f96f97aae98f3be2/examples/image.rs
pub fn load_image(path: &str) -> (Reader<File>, Vec<u32>) {
    let mut decoder = Decoder::new(File::open(path).unwrap());

    decoder.set_transformations(Transformations::ALPHA);
    let mut reader = decoder.read_info().unwrap();

    let mut image = vec![0u32; reader.output_buffer_size()];

    let mut u8_buffer = unsafe {
        std::slice::from_raw_parts_mut(
            image.as_mut_ptr() as *mut u8,
            image.len() * std::mem::size_of::<u32>(),
        )
    };

    reader.next_frame(&mut u8_buffer).unwrap();

    (reader, image)
}

