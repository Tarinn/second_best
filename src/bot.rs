use crate::{game::*, player::Player};

static SEARCH_DEPTH: usize = 5;

enum MoveResult {
    End(EndState),
    Continue(MoveNode),
}

struct MoveNode {
    board: Board,
    possible_turns: Vec<(Turn, MoveResult)>,
    score: (f64, f64, f64), // (White wins, Black wins, draws)
}

pub struct Bot {
    colour: Colour,
}

impl Player for Bot {
    fn get_colour(&self) -> Colour {
        self.colour
    }

    fn ask_put_piece(&self, board: &Board, second_best: bool) -> usize {
        let start_depth = board.count_pieces();

        let mut node = MoveNode {
            board: board.clone(),
            possible_turns: Vec::new(),
            score: (0.0, 0.0, 0.0),
        };

        Self::process_node(None, &mut node, start_depth, start_depth + SEARCH_DEPTH);

        let best_moves = self.best_move(node);
        let best_move = if !second_best {
            best_moves.0
        } else {
            best_moves.1
        };
        if let Turn::Place(_, i) = best_move {
            return i;
        } else {
            panic!("Something went wrong with bot.");
        }
    }

    fn ask_move_piece(&self, board: &Board, second_best: bool) -> (usize, usize) {
        let mut node = MoveNode {
            board: board.clone(),
            possible_turns: Vec::new(),
            score: (0.0, 0.0, 0.0),
        };

        let start_depth = 16;
        Self::process_node(None, &mut node, start_depth, start_depth + SEARCH_DEPTH);

        let best_moves = self.best_move(node);
        let best_move = if !second_best {
            best_moves.0
        } else {
            best_moves.1
        };
        if let Turn::Move(_, i, j) = best_move {
            return (i, j);
        } else {
            panic!("Something went wrong with bot.");
        }
    }

    fn ask_second_best(&self, _board: &Board, _turn: &Turn) -> bool {
        // TODO
        rand::random()
    }
}

impl Bot {
    pub fn new(colour: Colour) -> Self {
        Self { colour }
    }

    fn best_move(&self, node: MoveNode) -> (Turn, Turn) {
        let mut best = &node.possible_turns[0];
        let mut second_best = best;

        let mut best_score = 0.0;
        let mut second_best_score = 0.0;

        for i in 1..node.possible_turns.len() {
            let trial = &node.possible_turns[i];

            if self.colour == Colour::White {
                if let MoveResult::End(EndState::Win(Colour::White)) = trial.1 {
                    second_best = best;
                    best = trial;

                    second_best_score = best_score;
                    best_score = f64::MAX
                } else if let MoveResult::Continue(new_node) = &trial.1 {
                    if new_node.score.0 > best_score {
                        second_best = best;
                        best = trial;

                        second_best_score = best_score;
                        best_score = new_node.score.0;
                    } else if new_node.score.0 > second_best_score {
                        second_best = trial;
                        second_best_score = new_node.score.0;
                    }
                }
            } else {
                if let MoveResult::End(EndState::Win(Colour::Black)) = trial.1 {
                    second_best = best;
                    best = trial;

                    second_best_score = best_score;
                    best_score = f64::MAX
                } else if let MoveResult::Continue(new_node) = &trial.1 {
                    if new_node.score.1 > best_score {
                        second_best = best;
                        best = trial;

                        second_best_score = best_score;
                        best_score = new_node.score.1;
                    } else if new_node.score.1 > second_best_score {
                        second_best = trial;
                        second_best_score = new_node.score.1;
                    }
                }
            }
        }
        (best.0.clone(), second_best.0.clone())
    }

    fn process_node(root: Option<&MoveNode>, node: &mut MoveNode, depth: usize, max_depth: usize) {
        let colour = if depth % 2 == 0 {
            Colour::White
        } else {
            Colour::Black
        };
        if depth < max_depth && depth < 16 {
            for i in 0..8 {
                let turn = Turn::Place(colour, i);
                if node.board.is_possible_turn(&turn) {
                    Self::try_turn(turn, root, node, depth, max_depth);
                }
            }
        } else if depth < max_depth && depth >= 16 {
            for i in 0..8 {
                if *node.board.0[i].peek_top() == Piece::Piece(colour) {
                    for j in [1, 4, 7] {
                        let turn = Turn::Move(colour, i, j);
                        if node.board.is_possible_turn(&turn) {
                            Self::try_turn(turn, root, node, depth, max_depth);
                        }
                    }
                }
            }
        }
    }

    fn try_turn(
        turn: Turn,
        root: Option<&MoveNode>,
        node: &mut MoveNode,
        depth: usize,
        max_depth: usize,
    ) {
        let root = if let Some(n) = root { n } else { &node };

        let mut board = node.board.clone();
        board.do_turn(&turn);
        match board.is_won() {
            Some(endstate) => {
                match endstate {
                    EndState::Win(Colour::White) => {
                        node.score = (node.score.0 + 1.0, node.score.1, node.score.2);
                    }
                    EndState::Win(Colour::Black) => {
                        node.score = (node.score.0, node.score.1 + 1.0, node.score.2);
                    }
                    EndState::Draw => {
                        node.score = (node.score.0, node.score.1, node.score.2 + 1.0);
                    }
                }
                node.possible_turns.push((turn, MoveResult::End(endstate)));
            }
            None => {
                let mut new_node = MoveNode {
                    board,
                    possible_turns: Vec::new(),
                    score: (0.0, 0.0, 0.0),
                };
                Self::process_node(Some(root), &mut new_node, depth + 1, max_depth);
                node.score = (
                    node.score.0 + (new_node.score.0 / 2.0),
                    node.score.1 + (new_node.score.1 / 2.0),
                    node.score.2 + (new_node.score.2 / 2.0),
                );
                node.possible_turns
                    .push((turn, MoveResult::Continue(new_node)));
            }
        }
    }
}
