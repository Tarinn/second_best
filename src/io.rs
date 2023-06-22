use crate::game::*;
use std::io;

pub struct IO();

impl IO {
    pub fn start_game() -> Option<Colour> {
        println!("Welcome to 'Second Best', follow the instructions to start a game. White always starts.");
        loop {
            println!("Play against bot as White or Black (w/b) or against another player (p)?");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => match input.trim() {
                    "w" => return Some(Colour::White),
                    "b" => return Some(Colour::Black),
                    "p" => return None,
                    _ => {
                        println!("Invalid input");
                    }
                },
                Err(error) => println!("error: {error}"),
            }
        }
    }

    pub fn invalid_turn() {
        println!("That move is not possible, try again.");
    }

    pub fn bot_turn(turn: &Turn) {
        let move_string = match *turn {
            Turn::Place(colour, place) => {
                format!("placed a {:?} piece at {:?}", colour, place)
            }
            Turn::Move(colour, from_place, to_place) => {
                format!(
                    "moved a {:?} piece from {:?} to {:?}",
                    colour, from_place, to_place
                )
            }
        };
        println!("Computer has {:?}.", move_string);
    }

    pub fn ask_second_best() -> bool {
        loop {
            println!("Second best? (y/n):");

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => match input.trim() {
                    "y" => return true,
                    "n" => return false,
                    _ => {
                        println!("Invalid input");
                    }
                },
                Err(error) => println!("error: {error}"),
            }
        }
    }

    pub fn result_second_best(b: bool) {
        if b {
            println!("Second best! Try a new move.")
        }
    }

    pub fn ask_move_piece(colour: Colour) -> (usize, usize) {
        loop {
            println!("{:?}, move a piece. (1-8) (1-8): ", colour);

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    input = input.trim().to_owned();
                    match (input.chars().nth(0), input.chars().nth(2)) {
                        (Some(a), Some(b)) => {
                            match (
                                a.to_string().parse::<usize>(),
                                b.to_string().parse::<usize>(),
                            ) {
                                (Ok(n), Ok(m)) => {
                                    if n >= 1 && n <= 8 && m >= 1 && m <= 8 {
                                        return (n - 1, m - 1);
                                    } else {
                                        println!("Invalid input");
                                    }
                                }
                                _ => {
                                    println!("Invalid input");
                                }
                            }
                        }
                        _ => {
                            println!("Invalid input");
                        }
                    }
                }
                Err(error) => println!("error: {error}"),
            }
        }
    }

    pub fn ask_put_piece(colour: Colour) -> usize {
        loop {
            println!("{:?}, place a piece. (1-8): ", colour);

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => match input.trim().parse::<usize>() {
                    Ok(n) => {
                        if n >= 1 && n <= 8 {
                            return n - 1;
                        } else {
                            println!("Invalid input");
                        }
                    }
                    Err(_) => {
                        println!("Invalid input");
                    }
                },
                Err(error) => println!("error: {error}"),
            }
        }
    }

    pub fn end_game(endstate: EndState) {
        match endstate {
            EndState::Win(colour) => {
                println!("{:?} has won the game!", colour);
            }
            EndState::Draw => {
                println!("The game is a draw.");
            }
        }
    }

    pub fn piece_string(piece: &Piece) -> String {
        match piece {
            Piece::Blank => " ".to_owned(),
            Piece::Piece(Colour::White) => "□".to_owned(),
            Piece::Piece(Colour::Black) => "■".to_owned(),
        }
    }

    pub fn place_string(place: &Place) -> String {
        format!(
            "{}{}{}",
            Self::piece_string(&place.0[0]),
            Self::piece_string(&place.0[1]),
            Self::piece_string(&place.0[2])
        )
    }

    pub fn print_board(board: &Board) {
        println!(
            "4    [{}] [{}]    5",
            Self::place_string(&board.0[3]),
            Self::place_string(&board.0[4])
        );
        println!(
            "3   [{}]   [{}]   6",
            Self::place_string(&board.0[2]),
            Self::place_string(&board.0[5])
        );
        println!(
            "2   [{}]   [{}]   7",
            Self::place_string(&board.0[1]),
            Self::place_string(&board.0[6])
        );
        println!(
            "1    [{}] [{}]    8",
            Self::place_string(&board.0[0]),
            Self::place_string(&board.0[7])
        );
    }
}
