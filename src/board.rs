use rand;

#[derive(Debug)]
pub struct Board(pub Vec<Vec<bool>>);

impl Board {
    pub fn update(&mut self) {
        let width: i32 = self.0[0].len() as i32;
        let height: i32 = self.0.len() as i32;
        let mut new_board: Vec<Vec<bool>> = create_board(width, height, false).0;

        for row in 0..height {
            for col in 0..width {
                let mut neighbour_count: u8 = 0;

                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }

                        if 0 <= col + i && col + i < width && 0 <= row + j && row + j < height && self.0[(row + j) as usize][(col + i) as usize] {
                            neighbour_count += 1;
                        }
                    }
                }

                new_board[row as usize][col as usize] = match neighbour_count {
                    2 => self.0[row as usize][col as usize],
                    3 => true,
                    _ => false
                }
            }
        }

        *self = Board(new_board)
    }
}

pub fn create_board(width: i32, height: i32, randomize: bool) -> Board {
    let mut result: Vec<Vec<bool>> = Vec::new();

    for _ in 0..height {
        let mut row: Vec<bool> = Vec::new();

        for _ in 0..width {
            if randomize {
                row.push(rand::random_bool(0.5));
            } else {
                row.push(false);
            }
        }

        result.push(row);
    }

    Board(result)
}