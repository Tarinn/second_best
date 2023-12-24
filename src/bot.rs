use std::{rc::Rc, iter, borrow::Borrow};

use rand::seq::SliceRandom;

use crate::{game::*, player::Player, io::IO};

pub struct Bot {
    colour: Colour,
    search_depth: u64
}

impl Player for Bot {
    fn get_colour(&self) -> Colour {
        self.colour
    }

    fn ask_put_piece(&self, board: &Board, second_best: bool) -> usize {
        if let Turn::Place(_, i) = self.best_turn(board, second_best) {
            return i;
        } else {
            panic!("Bot encoured invalid board state.")
        }
    }

    fn ask_move_piece(&self, board: &Board, second_best: bool) -> (usize, usize) {
        if let Turn::Move(_, i, j) = self.best_turn(board, second_best) {
            return (i, j);
        } else {
            panic!("Bot encoured invalid board state.")
        }
    }

    fn ask_second_best(&self, board: &Board, turn: &Turn) -> bool {
        self.best_turns_for_colour(board, self.colour.opposite(), false).contains(turn)
    }
}

impl Bot {
    pub fn new(colour: Colour, search_depth: u64) -> Self {
        Self { 
            colour,
            search_depth
        }
    }

    // Return vec of all turns with (second) best score for a colour
    fn best_turns_for_colour(&self, board: &Board, colour: Colour, second_best: bool) -> Vec<Turn> {
        // Create a vector of all possible turns
        let binding = if board.count_pieces() < 16 {
            Self::all_possible_place(colour, &board)
        } else {
            Self::all_possible_move(colour, &board)
        };

        // Score each move
        let possible_turns: Vec<(Turn, f64)> = binding.into_iter().map(|turn| (turn.clone(), self.score_turn(&turn, board, colour, false, self.search_depth))).collect();

        let mut best_moves: (Vec<Turn>, f64);
        let mut second_best_moves: (Vec<Turn>, f64);
        if possible_turns[0].1 > possible_turns[1].1 {
            best_moves = (vec![possible_turns[0].0.clone()], possible_turns[0].1);
            second_best_moves = (vec![possible_turns[1].0.clone()], possible_turns[1].1);
        } else {
            best_moves = (vec![possible_turns[1].0.clone()], possible_turns[1].1);
            second_best_moves = (vec![possible_turns[0].0.clone()], possible_turns[0].1);
        }
        for i in 2..possible_turns.len() {
            let turn_with_score = &possible_turns[i];

            if turn_with_score.1 > best_moves.1 {
                second_best_moves = best_moves;
                best_moves = (vec![turn_with_score.0.clone()], turn_with_score.1);
            } else if turn_with_score.1 == best_moves.1 {
                best_moves.0.push(turn_with_score.0.clone());
            } else if turn_with_score.1 > second_best_moves.1 {
                second_best_moves = (vec![turn_with_score.0.clone()], turn_with_score.1);
            } else if turn_with_score.1 == second_best_moves.1 {
                second_best_moves.0.push(turn_with_score.0.clone())
            }
        }
        if second_best {
            second_best_moves.0
        } else {
            best_moves.0
        }
    }

    // Return the (second) best turn for the bot
    fn best_turn(&self, board: &Board, second_best: bool) -> Turn {
        self.best_turns_for_colour(board, self.colour, second_best).choose(&mut rand::thread_rng()).unwrap().clone()
    }

    fn score_turn(&self, turn: &Turn, board: &Board, colour: Colour, opponent: bool, depth: u64) -> f64 {
        let mut new_board = board.clone();
        new_board.do_turn(turn);

        // If the board is in an endstate; return score based on it
        if let Some(endstate) = new_board.is_won() {
            (match endstate {
                EndState::Win(Colour::Black) => {
                    if colour == Colour::Black {
                        100.0
                    } else {
                        -100.0
                    }
                },
                EndState::Win(Colour::White) => {
                    if colour == Colour::White {
                        100.0
                    } else {
                        -100.0
                    }
                },
                EndState::Draw => 0.0,
            }) * (if opponent { -1.0 } else { 1.0 })
        } else { // If not in an endstate; return 0 if searchdepth is reached, otherwise average score of next moves
            if depth == 0 {
                0.0
            } else {
                let binding = if new_board.count_pieces() < 16 {
                    Self::all_possible_place(self.colour, &new_board)
                } else {
                    Self::all_possible_move(self.colour, &new_board)
                };
                binding.iter().map(|turn| self.score_turn(turn, &new_board, colour.opposite(), !opponent, depth - 1)).sum::<f64>() / (binding.len() as f64)
            }
        }
    }

    fn all_possible_place(colour: Colour, board: &Board) -> Vec<Turn> {
        let mut turns: Vec<Turn> = vec![];
        for i in 0..8 {
            let turn = Turn::Place(colour, i);
            if board.is_possible_turn(&turn) {
                turns.push(turn);
            }
        }
        turns
    }

    fn all_possible_move(colour: Colour, board: &Board) -> Vec<Turn> {
        let mut turns: Vec<Turn> = vec![];
        for i in 0..8 {
            for j in [1, 4, 7] {
                let turn = Turn::Move(colour, i, (i + j) % 8);
                if board.is_possible_turn(&turn) {
                    turns.push(turn);
                }
            }
        }
        turns
    }
}
