use lenia_ca::{kernels, lenias::StandardLenia, Kernel, Simulator};
use macroquad::prelude::*;
use ndarray::ArrayD;
use ndarray_rand::{rand_distr::Uniform, RandomExt};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

const CELL_SIZE: usize = 4;

fn window_conf() -> Conf {
    Conf {
        window_title: "Lenia".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let window_width = screen_width() as usize;
    let window_height = screen_height() as usize;

    let cell_size = CELL_SIZE;

    let grid_width = window_width / cell_size;
    let grid_height = window_height / cell_size;

    println!(
        "Window size: {}x{}, Grid size: {}x{}",
        window_width, window_height, grid_width, grid_height
    );

    let mut image = Image::gen_image_color(window_width as u16, window_height as u16, WHITE);
    let texture = Texture2D::from_image(&image);

    let starting_pattern = ArrayD::random(
        ndarray::IxDyn(&[grid_width, grid_height]),
        Uniform::new(0., 1.),
    );

    let channel_shape: Vec<usize> = vec![grid_width, grid_height];

    let mut simulator = Simulator::<StandardLenia>::new(&channel_shape);
    simulator.fill_channel(&starting_pattern, 0);

    loop {
        simulator.iterate();

        let channel = simulator.get_channel_as_ref(0);

        for x in 0..grid_width {
            for y in 0..grid_height {
                let val = (channel[[x, y]] * 255.0) as u8;
                let color = Color::from_rgba(val, val, val, 255);

                for px in x * cell_size..(x + 1) * cell_size {
                    for py in y * cell_size..(y + 1) * cell_size {
                        image.set_pixel(px as u32, py as u32, color);
                    }
                }
            }
        }

        texture.update(&image);
        draw_texture(&texture, 0., 0., WHITE);
        next_frame().await;

        simulator.set_kernel(
            kernels::gaussian_donut_2d(rand::gen_range(2, 20), 1.0 / 6.7),
            0,
        );
    }
}
