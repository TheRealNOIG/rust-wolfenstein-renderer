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
/// - `height`: The height of the wall section to draw.
/// - `ceiling_color`: The color of the ceiling section.
/// - `wall_color`: The color of the wall section.
/// - `floor_color`: The color of the floor section.
/// - `buffer`: A mutable reference to the screen buffer where pixels are drawn.
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
    for y in wall_start..std::cmp::min(wall_end, buffer_height) {
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
/// - `map`: A reference to the 2D grid map, represented as a linear array.
///
/// # Returns:
/// The distance to the first obstacle hit by the ray, or the maximum distance if no obstacle is encountered.
pub fn slow_raycast(
    start_x: f32,
    start_y: f32,
    theta: f32,
    step_increment: f32,
    max_distance: f32,
    map_width: usize,
    map_height: usize,
    map: &[u8],
) -> f32 {
    let mut distance = 0.0;
    let (mut x, mut y) = (start_x, start_y);
    let (dx, dy) = (theta.cos() * step_increment, theta.sin() * step_increment);

    while distance < max_distance {
        // Calculate the grid position
        let grid_x = x as usize;
        let grid_y = y as usize;

        // Early exit if out of bounds
        if grid_x >= map_width || grid_y >= map_height {
            break;
        }
        // Check for a hit
        if map[grid_x + grid_y * map_width] == 1 {
            return distance;
        }

        // Move the ray forward
        x += dx;
        y += dy;
        distance += step_increment;
    }

    distance
}

