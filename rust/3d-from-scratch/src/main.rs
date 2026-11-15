mod obj;

use minifb::{Key, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

const BACKGROUND: u32 = 0xAAAAAA;
const FOREGROUND: u32 = 0x101010;

const FPS: u32 = 60;

const MODEL: &str = include_str!("./sap_avali.obj");

fn main() {
    let model = obj::parse(&MODEL);

    let mut window = Window::new(
        "3D from Scratch",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let vertices: Vec<Point3D> = model
        .vertices
        .iter()
        .map(|v| Point3D {
            x: v.x,
            y: v.y,
            z: v.z,
        })
        .collect();

    let faces: Vec<Vec<usize>> = model
        .faces
        .iter()
        .map(|f| f.vertex_indexes.clone())
        .collect();

    let facesa = vec![
        vec![0, 1, 2, 3],
        vec![4, 5, 6, 7],
        vec![0, 4],
        vec![1, 5],
        vec![2, 6],
        vec![3, 7],
    ];

    let delta_position = Point3D {
        x: 0.0,
        y: -0.6,
        z: 1.0,
    };
    let mut angle = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // TODO: Schedule fps sleep
        let delta_time = 1.0 / FPS as f32;
        angle += std::f32::consts::PI * delta_time;

        clear(&mut buffer);

        for f in &faces {
            for i in 0..f.len() {
                let a = &vertices[f[i]];
                let b = &vertices[f[(i + 1) % f.len()]];

                line(
                    &screen(project(&translate(&rotate_xz(&a, angle), &delta_position))),
                    &screen(project(&translate(&rotate_xz(&b, angle), &delta_position))),
                    &mut buffer,
                );
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn clear(buffer: &mut Vec<u32>) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            buffer[x + y * WIDTH] = BACKGROUND;
        }
    }
}

fn point(point: &ScreenPoint, buffer: &mut Vec<u32>) {
    let size = 20;

    let point_x = point.x as i32 - size / 2 as i32;
    let point_y = point.y as i32 - size / 2 as i32;

    for x in point_x..(point_x + size) {
        for y in point_y..(point_y + size) {
            if x < 0 || y < 0 || x >= WIDTH as i32 || y >= HEIGHT as i32 {
                continue;
            }

            let index = x as usize + y as usize * WIDTH;

            buffer[index] = FOREGROUND;
        }
    }
}

fn line(point1: &ScreenPoint, point2: &ScreenPoint, buffer: &mut Vec<u32>) {
    draw_line(
        buffer,
        point1.x as i32,
        point1.y as i32,
        point2.x as i32,
        point2.y as i32,
        FOREGROUND,
    );
}

struct Point {
    x: f32,
    y: f32,
}

struct ScreenPoint {
    x: usize,
    y: usize,
}

fn screen(point: Point) -> ScreenPoint {
    let px = (((point.x + 1.0) / 2.0) * WIDTH as f32) as usize;
    let py = ((1.0 - (point.y + 1.0) / 2.0) * HEIGHT as f32) as usize;

    ScreenPoint { x: px, y: py }
}

#[derive(Clone)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

fn project(point: &Point3D) -> Point {
    Point {
        x: point.x / point.z,
        y: point.y / point.z,
    }
}

fn translate(point: &Point3D, delta: &Point3D) -> Point3D {
    Point3D {
        x: point.x + delta.x,
        y: point.y + delta.y,
        z: point.z + delta.z,
    }
}

fn rotate_xz(point: &Point3D, angle: f32) -> Point3D {
    let c = angle.cos();
    let s = angle.sin();
    Point3D {
        x: point.x * c - point.z * s,
        y: point.y,
        z: point.x * s + point.z * c,
    }
}

/// Bresenham's line algorithm
fn draw_line(buffer: &mut Vec<u32>, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx + dy;
    let (mut x, mut y) = (x0, y0);

    loop {
        if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
            buffer[x as usize + y as usize * WIDTH] = color;
        }

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 >= dy {
            err += dy;
            x += sx;
        }

        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
