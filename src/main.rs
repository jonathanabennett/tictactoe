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
    } else if cells[0] == cells[4] && cells[4] == cells[8] {
        cells[0]
    } else if cells[0] == cells[3] && cells[3] == cells[6] {
        cells[0]
    } else if cells[1] == cells[4] && cells[4] == cells[7] {
        cells[1]
    } else if cells[2] == cells[5] && cells[5] == cells[8] {
        cells[2]
    } else if cells[3] == cells[4] && cells[4] == cells[5] {
        cells[3]
    } else if cells[6] == cells[4] && cells[4] == cells[2] {
        cells[6]
    } else if cells[6] == cells[7] && cells[7] == cells[8] {
        cells[8]
    } else {
        0
    }
}
