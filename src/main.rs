use minifb::{Key, Window, WindowOptions};
use std::env;

mod board;
mod utils;

fn main() {
    let args: utils::BoardProperties = utils::argument_parser(env::args().collect());

    if args.send_help {
        utils::print_help();
        return;
    }

    let mut game: board::Board = board::create_board(
        args.width / args.cell_size,
        args.height / args.cell_size,
        true,
    );

    let mut window = Window::new(
        "Game of Life",
        args.width,
        args.height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_target_fps(args.frame_rate);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut buffer: Vec<u32> = vec![0; args.width * args.height];

        for (row, col_vec) in game.0.iter().enumerate() {
            for (col, value) in col_vec.iter().enumerate() {
                if *value {
                    for i in (col * args.cell_size)..((col + 1) * args.cell_size) {
                        for j in (row * args.cell_size)..((row + 1) * args.cell_size) {
                            buffer[j * args.width + i] = 0x00ffffff;
                        }
                    }
                }
            }
        }

        window
            .update_with_buffer(&buffer, args.width, args.height)
            .unwrap();
        game.update();
    }
}
