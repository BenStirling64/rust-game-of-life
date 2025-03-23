use rand;

pub struct Board(pub Vec<Vec<bool>>);

impl Board {
    pub fn update(&mut self) {
        let width: usize = self.0[0].len();
        let height: usize = self.0.len();
        let mut new_board: Vec<Vec<bool>> = create_board(width, height, false).0;

        for row in 0..height as isize {
            for col in 0..width as isize {
                let mut neighbour_count: u8 = 0;

                for i in -1..=1 as isize {
                    for j in -1..=1 as isize {
                        if i == 0 && j == 0 {
                            continue;
                        }

                        if 0 <= col + i
                            && col + i < width.try_into().unwrap()
                            && 0 <= row + j
                            && row + j < height.try_into().unwrap()
                            && self.0[(row + j) as usize][(col + i) as usize]
                        {
                            neighbour_count += 1;
                        }
                    }
                }

                new_board[row as usize][col as usize] = match neighbour_count {
                    2 => self.0[row as usize][col as usize],
                    3 => true,
                    _ => false,
                }
            }
        }

        *self = Board(new_board)
    }
}

pub fn create_board(width: usize, height: usize, randomize: bool) -> Board {
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
