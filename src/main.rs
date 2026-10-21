use macroquad::prelude::*;
use macroquad::rand::gen_range;

const MOON_RADIUS: f32 = 100.0;
const CRATER_COUNT: usize = 35;

const SPACE_COLOR: Color = Color::new(0.02, 0.03, 0.08, 1.0);

// Each crater stores its position relative to the Moon
// and its radius.
struct Crater {
    x: f32,
    y: f32,
    radius: f32,
}

// Generate random craters inside a circular Moon
fn generate_craters(count: usize, moon_radius: f32) -> Vec<Crater> {
    let mut craters = Vec::new();

    for _ in 0..count {
        let radius = gen_range(3.0, 18.0);

        // Available area so craters don't cross the Moon's edge
        let max_distance = moon_radius - radius;

        loop {
            let x = gen_range(-max_distance, max_distance);
            let y = gen_range(-max_distance, max_distance);

            // Check that the crater fits inside the Moon
            if x * x + y * y <= max_distance * max_distance {
                craters.push(Crater { x, y, radius });
                break;
            }
        }
    }

    craters
}

#[macroquad::main("Procedural Moon")]
async fn main() {
    let craters = generate_craters(CRATER_COUNT, MOON_RADIUS);

    loop {
        clear_background(SPACE_COLOR);

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        let offset_y = (get_time() as f32 * 2.0).sin() * 20.0;
        // Base Moon
        draw_circle(
            center_x,
            // center_y,
            center_y + offset_y,
            MOON_RADIUS,
            LIGHTGRAY,
        );

        // Draw generated craters
        for crater in &craters {
            draw_circle(
                center_x + crater.x,
                center_y + crater.y,
                crater.radius,
                Color::from_rgba(43, 48, 45, 128),
            );
        }

        // Press SPACE to generate a new Moon
        // (See the small change below)

        next_frame().await;
    }
}