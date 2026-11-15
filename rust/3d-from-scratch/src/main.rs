use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut window = Window::new("Pixel Demo", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    // Draw a red pixel.
    buffer[100 * WIDTH + 100] = 0xFF0000;

    // Draw a green rectangle.
    for y in 200..300 {
        for x in 200..300 {
            buffer[y * WIDTH + x] = 0x00FF00;
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
