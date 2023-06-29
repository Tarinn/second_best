use crate::bot::*;
use crate::io::*;
use crate::player::Person;
use crate::player::Player;

use core::panic;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Colour {
    White,
    Black,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Piece {
    Piece(Colour),
    Blank,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Place(pub [Piece; 3]);

impl Place {
    pub fn new() -> Self {
        Self([Piece::Blank, Piece::Blank, Piece::Blank])
    }

    pub fn is_full(&self) -> bool {
        match &self.0 {
            [Piece::Piece(_), Piece::Piece(_), Piece::Piece(_)] => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.0 {
            [Piece::Blank, Piece::Blank, Piece::Blank] => true,
            _ => false,
        }
    }

    pub fn peek_top(&self) -> &Piece {
        match &self.0 {
            [Piece::Blank, Piece::Blank, Piece::Blank] => &Piece::Blank,
            [top @ Piece::Piece(_), Piece::Blank, Piece::Blank] => top,
            [Piece::Piece(_), top @ Piece::Piece(_), Piece::Blank] => top,
            [Piece::Piece(_), Piece::Piece(_), top @ Piece::Piece(_)] => top,
            _ => panic!("Board is invalid at {:?}", self),
        }
    }

    pub fn count_pieces(&self) -> usize {
        let mut total = 0;
        for i in 0..3 {
            if self.0[i] != Piece::Blank {
                total += 1;
            };
        }
        total
    }

    pub fn add_piece(&mut self, colour: &Colour) {
        match &self.0 {
            [Piece::Blank, Piece::Blank, Piece::Blank] => self.0[0] = Piece::Piece(*colour),
            [Piece::Piece(_), Piece::Blank, Piece::Blank] => self.0[1] = Piece::Piece(*colour),
            [Piece::Piece(_), Piece::Piece(_), Piece::Blank] => self.0[2] = Piece::Piece(*colour),
            _ => panic!(
                "Adding piece is invalid move, or board is invalid at {:?}",
                self
            ),
        }
    }

    pub fn remove_piece(&mut self, colour: &Colour) {
        match &self.0 {
            [Piece::Piece(c), Piece::Blank, Piece::Blank] if *c == *colour => {
                self.0[0] = Piece::Blank
            }
            [Piece::Piece(_), Piece::Piece(c), Piece::Blank] if *c == *colour => {
                self.0[1] = Piece::Blank
            }
            [Piece::Piece(_), Piece::Piece(_), Piece::Piece(c)] if *c == *colour => {
                self.0[2] = Piece::Blank
            }
            _ => panic!(
                "Removing piece is invalid move, or board is invalid at {:?}",
                self
            ),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum EndState {
    Win(Colour),
    Draw,
}

#[derive(Clone)]
pub struct Board(pub [Place; 8]);

impl Board {
    pub fn new() -> Self {
        Self([
            Place::new(), Place::new(), Place::new(), Place::new(),
            Place::new(), Place::new(), Place::new(), Place::new(),
        ])
    }

    pub fn count_pieces(&self) -> usize {
        let mut total = 0;
        for i in 0..8 {
            total += self.0[i].count_pieces();
        }
        total
    }

    pub fn is_won(&self) -> Option<EndState> {
        let mut black_win = false;
        let mut white_win = false;
        for i in 0..8 {
            match &self.0[i].0 {
                [Piece::Piece(Colour::Black), Piece::Piece(Colour::Black), Piece::Piece(Colour::Black)] => {
                    black_win = true
                }
                [Piece::Piece(Colour::White), Piece::Piece(Colour::White), Piece::Piece(Colour::White)] => {
                    white_win = true
                }
                _ => {}
            }
            match [
                &self.0[i].peek_top(),
                &self.0[(i + 1) % 8].peek_top(),
                &self.0[(i + 2) % 8].peek_top(),
                &self.0[(i + 3) % 8].peek_top(),
            ] {
                [&Piece::Piece(Colour::Black), &Piece::Piece(Colour::Black), 
                 &Piece::Piece(Colour::Black), &Piece::Piece(Colour::Black)] => {
                    black_win = true
                }
                [&Piece::Piece(Colour::White), &Piece::Piece(Colour::White), 
                 &Piece::Piece(Colour::White), &Piece::Piece(Colour::White)] => {
                    white_win = true
                }
                _ => {}
            }
        }
        match (black_win, white_win) {
            (true, true) => Some(EndState::Draw),
            (true, false) => Some(EndState::Win(Colour::Black)),
            (false, true) => Some(EndState::Win(Colour::White)),
            (false, false) => None,
        }
    }

    pub fn is_possible_turn(&self, turn: &Turn) -> bool {
        match *turn {
            Turn::Place(_, idx) => !self.0[idx].is_full(),
            Turn::Move(colour, idx1, idx2) => {
                if let Piece::Piece(c) = self.0[idx1].peek_top() {
                    !self.0[idx2].is_full()
                        && *c == colour
                        && (idx1 != idx2)
                        && (((idx1.abs_diff(idx2) % 8) == 1)
                            || ((idx1.abs_diff(idx2) % 8) == 7)
                            || ((idx1.abs_diff(idx2) % 8) == 4))
                } else {
                    false
                }
            }
        }
    }

    pub fn do_turn(&mut self, turn: &Turn) {
        match *turn {
            Turn::Place(colour, idx) => {
                self.0[idx].add_piece(&colour);
            }
            Turn::Move(colour, idx1, idx2) => {
                self.0[idx1].remove_piece(&colour);
                self.0[idx2].add_piece(&colour);
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Turn {
    Place(Colour, usize),
    Move(Colour, usize, usize),
}

pub struct Game {
    pub board: Board,
    pub turns: Vec<Turn>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turns: Vec::new(),
        }
    }

    pub fn start_game(&mut self) {
        match IO::start_game() {
            Some(Colour::White) => self.gameloop((
                Box::new(Person::new(Colour::White)),
                Box::new(Bot::new(Colour::Black)),
            )),
            Some(Colour::Black) => self.gameloop((
                Box::new(Bot::new(Colour::White)),
                Box::new(Person::new(Colour::Black)),
            )),
            None => self.gameloop((
                Box::new(Person::new(Colour::White)),
                Box::new(Person::new(Colour::Black)),
            )),
        }
    }

    fn gameloop(&mut self, players: (Box<dyn Player>, Box<dyn Player>)) {
        let mut turn_counter = 0;
        IO::print_board(&self.board);

        // Loop rounds of the game
        loop {
            // Determine the player making a turn each round (and the opponent deciding second best)
            let player = if turn_counter % 2 == 0 { &players.0 } else { &players.1 };
            let opponent = if turn_counter % 2 == 0 { &players.1 } else { &players.0 };

            // Stop the game if someone has won
            if let Some(winstate) = self.board.is_won() {
                IO::end_game(winstate);
                return;
            }

            // A turn in round is different depending on in placing or moving phase
            let try_turn = |b: bool| {
                if self.board.count_pieces() < 16 { // Placing
                    let p = player.ask_put_piece(&self.board, b);
                    Turn::Place(player.get_colour(), p)
                } else { // Moving
                    let m = player.ask_move_piece(&self.board, b);
                    Turn::Move(player.get_colour(), m.0, m.1)
                }
            };

            // Keep asking for turn until a valid one is given
            let mut turn: Turn;
            let mut valid_turn: bool;
            loop {
                turn = try_turn(false);
                valid_turn = self.board.is_possible_turn(&turn);
                if !valid_turn {
                    IO::invalid_turn();
                } else {
                    break;
                }
            }

            // Output the current board state, might change because of second best so temp board is made to apply move
            let mut temp_board = self.board.clone();
            temp_board.do_turn(&turn);
            IO::print_board(&temp_board);

            // Ask opponent for second best
            if opponent.ask_second_best(&self.board, &turn) {
                IO::result_second_best(true);

                // Demand valid turn again, but also cannot be equal to turn made in previous step
                let first_turn = turn.clone();
                loop {
                    turn = try_turn(true);
                    valid_turn = self
                        .board
                        .is_possible_turn(&turn)
                        && turn != first_turn;
                    if !valid_turn {
                        IO::invalid_turn();
                    } else {
                        break;
                    }
                }
            }

            // Apply the turn to the board
            self.board.do_turn(&turn);
            IO::print_board(&self.board);
            self.turns.push(turn);
            turn_counter += 1;
        }
    }
}