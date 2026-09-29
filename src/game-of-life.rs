use term_size;

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

fn count_neighbours(petri: &[Vec<bool>], row: usize, col: usize, rows: usize, cols: usize) -> i32 {
    let mut count = 0;

    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            if row_offset == 0 && col_offset == 0 {
                continue;
            }

            let neighbour_row = row as i32 + row_offset;
            let neighbour_col = col as i32 + col_offset;

            if neighbour_row < 0 || neighbour_row >= rows as i32 || neighbour_col < 0 || neighbour_col >= cols as i32 {
                continue;
            }

            if petri[neighbour_row as usize][neighbour_col as usize] {
                count += 1;
            }
        }
    }

    count
}

fn petri_simulate(old_petri: &[Vec<bool>], petri: &mut Vec<Vec<bool>>, rows: usize, cols: usize) {
    for row in 0..rows {
        for col in 0..cols {
            let neighbours = count_neighbours(old_petri, row, col, rows, cols);

            if old_petri[row][col] {
                petri[row][col] = neighbours == 2 || neighbours == 3;
            } else {
                petri[row][col] = neighbours == 3;
            }
        }
    }
}

fn main() {
    let (cols, rows) = term_size::dimensions().unwrap_or((80, 24));
    let mid_x = cols / 2;
    let mid_y = rows / 2;
    let mut generation_counter: i32 = 0;
    let mut petri = vec![vec![false; cols]; rows];

    if mid_y > 0 && mid_x > 0 && mid_y + 1 < rows && mid_x + 1 < cols {
        petri[mid_y - 1][mid_x] = true;
        petri[mid_y - 1][mid_x + 1] = true;
        petri[mid_y][mid_x - 1] = true;
        petri[mid_y][mid_x] = true;
        petri[mid_y + 1][mid_x] = true;
    }

    loop {
        let old_petri = petri.clone();
        petri_simulate(&old_petri, &mut petri, rows, cols);
        petri_print(&petri);
        println!("Generation: {}", generation_counter);
        generation_counter += 1;
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
