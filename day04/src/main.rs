#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct BingoNumber {
    value: u32,
    marked: bool,
}

impl BingoNumber {
    fn new(value: u32) -> Self {
        Self {
            value,
            marked: false,
        }
    }
}

type Board = [[BingoNumber; 5]; 5];

#[derive(Debug)]
struct Bingo {
    board: Board,
    winning_number: u32,
}

fn main() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let mut lines = input.lines();

    let drawn_numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|item| item.parse::<u32>().unwrap());

    let mut boards: Vec<Board> = Vec::new();

    let lines = lines.collect::<Vec<_>>();

    let mut start_line_idx = 0;

    while start_line_idx < lines.len() {
        let mut board: Board = Default::default();

        // Skip the empty line before each board
        start_line_idx += 1;

        for line_idx in 0..5 {
            let numbers = lines[start_line_idx + line_idx]
                .split_whitespace()
                .map(|item| BingoNumber::new(item.parse::<u32>().unwrap()))
                .collect::<Vec<_>>();

            board[line_idx] = numbers.try_into().unwrap();
        }

        // Skip the 5 processed board lines
        start_line_idx += 5;

        boards.push(board);
    }

    let mut bingo_found = true;
    let mut bingos: Vec<Bingo> = Vec::new();

    for drawn_number in drawn_numbers {
        for board in &mut boards {
            let mut has_bingo = false;

            // TODO: Make this lookup more efficient. HashSet?
            for bingo in &bingos {
                if bingo.board == *board {
                    has_bingo = true;
                }
            }

            if has_bingo {
                continue;
            }

            // Mark the drawn numbers
            for row in board.iter_mut() {
                for number in row {
                    if number.value == drawn_number {
                        number.marked = true;
                    }
                }
            }

            // Check for a row bingo
            for row in board.iter() {
                bingo_found = row.iter().all(|number| number.marked);

                if bingo_found {
                    break;
                }
            }

            if !bingo_found {
                // If row bingo is not found, check for a column bingo
                for column_idx in 0..5 {
                    bingo_found = true;

                    for row in board.iter() {
                        if !row[column_idx].marked {
                            bingo_found = false;
                            break;
                        }
                    }

                    if bingo_found {
                        break;
                    }
                }
            }

            if bingo_found {
                bingos.push(Bingo {
                    board: *board,
                    winning_number: drawn_number,
                });
            }
        }
    }

    let calculate_score = |bingo: &Bingo| {
        let mut unmarked_sum = 0;

        for row in bingo.board.iter() {
            for number in row {
                if !number.marked {
                    unmarked_sum += number.value;
                }
            }
        }

        unmarked_sum * bingo.winning_number
    };

    println!("Part 1:");
    println!(
        "\tThe score of the winning board is {}.",
        calculate_score(bingos.first().unwrap())
    );

    println!("Part 2:");
    println!(
        "\tThe score of the *last* winning board is {}.",
        calculate_score(bingos.last().unwrap())
    );
}
