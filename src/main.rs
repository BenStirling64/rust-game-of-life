use std::{env, process, thread, time};
use macroquad::prelude::*;

mod board;
mod utils;

fn window_config() -> Conf {
    let args: utils::BoardProperties = utils::argument_parser(env::args().collect());

    if args.send_help {
        utils::print_help();
        process::exit(0);
    }

    Conf {
        window_title: String::from("Game of Life"),
        window_width: args.width,
        window_height: args.height,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let args: utils::BoardProperties = utils::argument_parser(env::args().collect());
    let minimum_frame_length: f32 = 1000. / args.frame_rate; // in milliseconds
    let cell_size: i32 = args.cell_size;

    let mut game: board::Board = board::create_board(args.width / args.cell_size, args.height / args.cell_size, true);

    // println!("{game:?}");

    loop {
        let frame_length: f32 = get_frame_time() * 1000.;

        if frame_length < minimum_frame_length {
            thread::sleep(time::Duration::from_millis((minimum_frame_length - frame_length) as u64));
        }

        clear_background(BLACK);

        for (row, col_vec) in game.0.iter().enumerate() {
            for (col, value) in col_vec.iter().enumerate() {
                if *value {
                    draw_rectangle((col as i32 * cell_size) as f32, (row as i32 * cell_size) as f32, cell_size as f32, cell_size as f32, WHITE);
                }
            }
        }

        game.update();

        next_frame().await;
    }
}
