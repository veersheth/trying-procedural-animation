





fn main() {
    let cols = 20;
    let rows = 90;

    let mut petri: Vec<Vec<bool>> = Vec::new();

    for _ in 1..cols {
        let mut this_row: Vec<bool> = Vec::new();
        for _ in 1..rows {
            this_row.push(false);
        }
        petri.push(this_row);
    }
            

    for row in petri {
        for i in row {
            if i == false {
                print!("•");
            }
        }
        println!("");
    }
}


