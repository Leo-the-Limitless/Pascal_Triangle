use std::io;

fn print_spaces(rows: u32) {
    let spaces = (rows + rows - 1) / 2;
    for _ in 0..spaces {
        print!(" ");
    }
}

fn pascal(row: u32, col: u32)->u32 {
    if col == 0 || row == col {
        1
    } else {
        pascal(row-1, col-1) + pascal(row-1, col)
    }
} 

fn get_rows() -> u32 {
    let mut input = String::new();
    println!("Enter the number of rows:");
    io::stdin().read_line(&mut input).expect("Failed to Read");
    let rows = input.trim().parse().expect("Failed to Parse");
    rows
}

fn main() {
    let rows = get_rows();
    let mut spaces = rows;

    for row in 0..rows {
        print_spaces(spaces);
        for col in 0..=row {
            print!("{} ", pascal(row, col));
        }
        println!("");
        spaces -= 1;
    }
}
