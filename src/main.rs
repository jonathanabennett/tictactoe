use std::io;
use std::fmt;

#[derive(PartialEq)]
enum CellType {
    Player1,
    Player2,
    Empty(i32),
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &CellType::Player1 => write!(f, "X"),
            &CellType::Player2 => write!(f, "O"),
            &CellType::Empty(pos)   => write!(f,"{}", pos),
        }
    }
}

fn main() {
    //Varables
    let mut cells: [CellType; 9] = [CellType::Empty(0), CellType::Empty(1), CellType::Empty(2),
    CellType::Empty(3), CellType::Empty(4), CellType::Empty(5),
    CellType::Empty(6), CellType::Empty(7), CellType::Empty(8)];
    let mut player = CellType::Player1;
    let mut victor = CellType::Empty(99);

    while let CellType::Empty(_) = victor {
        print_board(&cells);

        change_cell(&mut cells, player);

        print_board(&cells);
        victor = check_victory(cells);
        if player == CellType::Player1 {
            player = CellType::Player2;
        } else {
            player = CellType::Player1;
        }
    }
    println!("Player {} wins!", victor);
}

fn print_board(cells: &[CellType; 9]) {
    let mut output: [char; 9];
    for cell in cells {

    println!("{}, {}, {}\n{}, {}, {}\n{}, {}, {}\n\n\n",
             cells[0], cells[1], cells[2],
             cells[3], cells[4], cells[5],
             cells[6], cells[7], cells[8]);
    }
}

fn change_cell(cells: &mut [CellType; 9], player: CellType) {
    let mut exit = 0;

    while exit == 0 {
        println!("Enter the number of the cell you wish to mark:");

        let mut cell = String::new();

        io::stdin().read_line(&mut cell)
            .expect("Please enter a number from 0-8.");

        let cell: usize = cell.trim().parse()
            .expect("Please enter a number from 0-8.");

        if cell < 9 {
            if let CellType::Empty(_) = cells[cell] {
                cells[cell] = player;
                exit = 1;
            } else {
                println!("That cell is already taken!");
            }
        } else {
            println!("That's not a valid move! You lose your turn.");
        }
    }
}

fn check_victory(cells: [CellType; 9]) -> CellType {
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
       CellType::Empty(99)
    }
}
