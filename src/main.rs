fn main() {
    println!("Hello, world!");

    //Varables
    let mut cells = [0,0,0,0,0,0,0,0,0];

    println!("{}, {}, {}\n{}, {}, {}\n{}, {}, {}",
             cells[0], cells[1], cells[2],
             cells[3], cells[4], cells[5],
             cells[6], cells[7], cells[8]);
    cells[0] = 1;
    cells[1] = 1;
    cells[2] = 1;
    println!("Player {} wins!", check_victory(cells));
}

fn check_victory(cells: [u32; 9]) -> u32 {
    if cells[0] == cells[1] && cells[1] == cells[2] {
        cells[0]
    } else {
        0
    }
}
