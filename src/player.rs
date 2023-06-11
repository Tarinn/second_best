use crate::{
    game::{Board, Colour, Turn},
    io::IO,
};

pub trait Player {
    fn get_colour(&self) -> Colour;
    fn ask_put_piece(&self, board: &Board, second_best: bool) -> usize;
    fn ask_move_piece(&self, board: &Board, second_best: bool) -> (usize, usize);
    fn ask_second_best(&self, board: &Board, turn: &Turn) -> bool;
}

pub struct Person {
    colour: Colour,
}

impl Person {
    pub fn new(colour: Colour) -> Self {
        Self { colour }
    }
}

impl Player for Person {
    fn get_colour(&self) -> Colour {
        self.colour
    }

    fn ask_put_piece(&self, _board: &Board, _second_best: bool) -> usize {
        IO::ask_put_piece(self.colour)
    }

    fn ask_move_piece(&self, _board: &Board, _second_best: bool) -> (usize, usize) {
        IO::ask_move_piece(self.colour)
    }

    fn ask_second_best(&self, _board: &Board, _turn: &Turn) -> bool {
        IO::ask_second_best()
    }
}
