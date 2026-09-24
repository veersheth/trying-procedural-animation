const COLS: usize = 110;
const ROWS: usize = 20;
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
                print!("•");
            } else {
                print!(" ");
            }
        }
        println!();
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
    petri[MID_Y][MID_X] = true;

    loop {
        petri_print(&petri);
    }
}



