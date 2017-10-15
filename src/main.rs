use std::io;

fn main() {
    println!("Hello, world!");

    //Varables
    let mut cells: [u8;9] = [0,0,0,0,0,0,0,0,0];
    let mut player: u8 = 1;
    let mut victor: u8 = 0;

    while victor == 0 {
        print_board(&cells);
        println!("Enter the number of the cell you wish to mark:");

        let mut cell = String::new();

        io::stdin().read_line(&mut cell)
            .expect("Please enter a number from 0-8.");

        let cell: usize = cell.trim().parse()
            .expect("Please enter a number from 0-8.");

        change_cell(&mut cells, cell, player);
        print_board(&cells);
        victor = check_victory(cells);
        if player == 1 {
            player = 2;
        } else {
            player = 1;
        }
    }
    println!("Player {} wins!", victor);
}

fn print_board(cells: &[u8; 9]) {
    println!("{}, {}, {}\n{}, {}, {}\n{}, {}, {}\n\n\n",
             cells[0], cells[1], cells[2],
             cells[3], cells[4], cells[5],
             cells[6], cells[7], cells[8]);
}

fn change_cell(cells: &mut [u8; 9], cell: usize, player: u8) -> u8 {
    if cell < 9 {
        if cells[cell] == 0 {
            cells[cell] = player;
            player
        } else {
            10
        }
    } else {
        println!("That's not a valid move! You lose your turn.");
        10
    }
}

fn check_victory(cells: [u8; 9]) -> u8 {
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
