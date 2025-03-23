pub struct BoardProperties {
    pub send_help: bool,
    pub width: usize,
    pub height: usize,
    pub cell_size: usize,
    pub frame_rate: usize,
}

pub fn argument_parser(arg_list: Vec<String>) -> BoardProperties {
    let mut properties = BoardProperties {
        send_help: false,
        width: 1600,
        height: 800,
        cell_size: 4,
        frame_rate: 60,
    };

    for (i, value) in arg_list[1..].iter().enumerate().step_by(2) {
        match value.as_str() {
            "--help" => {
                properties.send_help = true;
                break;
            }
            "-w" | "--width" => {
                properties.width = (&arg_list[i + 2]).parse::<usize>().unwrap_or(1600)
            }
            "-h" | "--height" => {
                properties.height = (&arg_list[i + 2]).parse::<usize>().unwrap_or(800)
            }
            "-c" | "--cell-size" => {
                properties.cell_size = (&arg_list[i + 2]).parse::<usize>().unwrap_or(4)
            }
            "-f" | "--frame-rate" => {
                properties.frame_rate = (&arg_list[i + 2]).parse::<usize>().unwrap_or(60)
            }
            _ => (),
        }
    }

    properties
}

pub fn print_help() {
    println!("Welcome to Conway's Game of Life!");
    println!("By default, the board has 400x200 cells (over a 1600x800 window), and each cell is 4x4 pixels large.");
    println!("The simulation targets a frame rate of 60 frames per second.");
    println!();
    println!("The options are as follows:");
    println!("'-w <width>' or '--width <width>' changes the width of the window to <width> pixels");
    println!(
        "'-h <height>' or '--height <height>' changes the height of the window to <height> pixels"
    );
    println!("'-c <cell_size>' or '--cell-size <cell_size>' changes the size of each cell to <cell_size>x<cell_size> pixels.");
    println!("'-f <fps>' or '--frame-rate <fps>' forces the simulation to target <fps> frames per second.");
}
