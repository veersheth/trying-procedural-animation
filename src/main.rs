const COLS: usize = 160;
const ROWS: usize = 40;
const MID_X: usize = COLS / 2;
const MID_Y: usize = ROWS / 2;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn petri_print(petri: &[Vec<bool>]) {
    clear_screen();
    for row in petri {
        for cell in row {
            if *cell {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn count_neighbours(petri: &[Vec<bool>], row: usize, col: usize) -> i32 {
    let mut count = 0;

    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            if row_offset == 0 && col_offset == 0 {
                continue;
            }

            let neighbour_row = row as i32 + row_offset;
            let neighbour_col = col as i32 + col_offset;

            if neighbour_row < 0 || neighbour_row >= ROWS as i32 || neighbour_col < 0 || neighbour_col >= COLS as i32 {
                continue;
            }

            if petri[neighbour_row as usize][neighbour_col as usize] {
                count += 1;
            }
        }
    }

    count
}

fn petri_simulate(old_petri: &[Vec<bool>], petri: &mut Vec<Vec<bool>>) {
    for row in 0..ROWS {
        for col in 0..COLS {
            let neighbours = count_neighbours(old_petri, row, col);

            if old_petri[row][col] {
                petri[row][col] = neighbours == 2 || neighbours == 3;
            } else {
                petri[row][col] = neighbours == 3;
            }
        }
    }
}

fn main() {
    let mut petri: Vec<Vec<bool>> = Vec::new();

    for _ in 0..ROWS {
        let mut this_row: Vec<bool> = Vec::new();
        for _ in 0..COLS {
            this_row.push(false);
        }
        petri.push(this_row);
    }

    petri[MID_Y - 1][MID_X] = true;
    petri[MID_Y - 1][MID_X + 1] = true;
    petri[MID_Y][MID_X - 1] = true;
    petri[MID_Y][MID_X] = true;
    petri[MID_Y + 1][MID_X] = true;

    loop {
        let old_petri = petri.clone();
        petri_simulate(&old_petri, &mut petri);
        petri_print(&petri);
        std::thread::sleep(std::time::Duration::from_millis(40));
    }
}
