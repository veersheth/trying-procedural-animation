

fn petri_print(petri: &Vec<Vec<bool>>) {
    for row in petri {
        for cell in row {
            if *cell == false {
                print!(" ");
            } else {
                print!("•");
            }
        }
        println!("");
    }
}

fn main() {
    let cols = 20;
    let rows = 90;

    let mut petri: Vec<Vec<bool>> = Vec::new();

    for _ in 0..cols {
        let mut this_row: Vec<bool> = Vec::new();
        for _ in 0..rows {
            this_row.push(false);
        }
        petri.push(this_row);
    }

    petri_print(&petri);
}


