use std::io;

#[derive(PartialEq)]
enum CellType {
    Player1,
    Player2,
    Empty,
}

impl CellType {
    fn output(self, pos:u8) {
        match self {
            CellType::Player1 => 'X',
            CellType::Player2 => 'O',
            CellType::Empty =>  pos.to_char()
        }
    }
}

fn main() {
    println!("Hello, world!");

    //Varables
    let mut cells: [CellType; 9] = [CellType::Empty, CellType::Empty, CellType::Empty,
    CellType::Empty, CellType::Empty, CellType::Empty,
    CellType::Empty, CellType::Empty, CellType::Empty];
    let mut player = CellType::Player1;
    let mut victor = CellType::Empty;

    while victor == CellType::Empty {
        print_board(&cells);
        println!("Enter the number of the cell you wish to mark:");

        let mut cell = String::new();

        io::stdin().read_line(&mut cell)
            .expect("Please enter a number from 0-8.");

        let cell: usize = cell.trim().parse()
            .expect("Please enter a number from 0-8.");

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
             cells[0].output(0), cells[1].output(1), cells[2].output(2),
             cells[3].output(3), cells[4].output(4), cells[5].output(5),
             cells[6].output(6), cells[7].output(7), cells[8].output(7));
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
            if cells[cell] == CellType::Empty {
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
       CellType::Empty
    }
}
