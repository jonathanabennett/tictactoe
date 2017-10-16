use std::io;

fn main() {
    println!("Hello, world!");

    //Varables
    let mut cells: [char; 9] = ['0','1','2','3','4','5','6','7','8'];
    let mut player: char = 'O';
    let mut victor: char = 'N';

    while victor == 'N' {
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
        if player == 'O' {
            player = 'X';
        } else {
            player = 'O';
        }
    }
    println!("Player {} wins!", victor);
}

fn print_board(cells: &[char; 9]) {
    println!("{}, {}, {}\n{}, {}, {}\n{}, {}, {}\n\n\n",
             cells[0], cells[1], cells[2],
             cells[3], cells[4], cells[5],
             cells[6], cells[7], cells[8]);
}

fn change_cell(cells: &mut [char; 9], cell: usize, player: char) {
    if cell < 9 {
        if cells[cell] != 'O' && cells[cell] != 'X' {
            cells[cell] = player;
        } else {
            println!("That cell is already taken!");
        }
    } else {
        println!("That's not a valid move! You lose your turn.");
    }
}

fn check_victory(cells: [char; 9]) -> char {
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
        'N'
    }
}
