use crate::{game::*, player::Player, io::IO};

// Depth to search for optimal move
static SEARCH_DEPTH: usize = 5;

enum MoveResult {
    End(EndState),
    Continue(MoveNode),
}

struct MoveNode {
    board: Board, // The board state in this node
    possible_turns: Vec<(Turn, MoveResult)>, // All possible moves from this board state
    score: (f64, f64, f64), // (White wins, Black wins, draws), combined results from possible moves
}

pub struct Bot {
    colour: Colour,
}

impl Player for Bot {
    fn get_colour(&self) -> Colour {
        self.colour
    }

    // TODO: should not be bool, instead Option<Turn>
    fn ask_put_piece(&self, board: &Board, second_best: bool) -> usize {
        let start_depth = board.count_pieces();

        let mut node = MoveNode {
            board: board.clone(),
            possible_turns: Vec::new(),
            score: (0.0, 0.0, 0.0),
        };

        Self::process_node(&mut node, start_depth, start_depth + SEARCH_DEPTH);

        let best_moves = self.best_move(node);
        let best_move = if !second_best {
            best_moves.0
        } else {
            best_moves.1
        };
        if let Turn::Place(_, i) = best_move {
            IO::bot_turn(&best_move);
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
        Self::process_node(&mut node, start_depth, start_depth + SEARCH_DEPTH);

        let best_moves = self.best_move(node);
        let best_move = if !second_best {
            best_moves.0
        } else {
            best_moves.1
        };
        if let Turn::Move(_, i, j) = best_move {
            IO::bot_turn(&best_move);
            return (i, j);
        } else {
            panic!("Something went wrong with bot.");
        }
    }

    fn ask_second_best(&self, board: &Board, turn: &Turn) -> bool {
        // Pretend to be opponent
        let opponent_colour = if self.colour == Colour::White {Colour::Black} else {Colour::White};
        let opponent_bot = Bot::new(opponent_colour);

        let mut node = MoveNode {
            board: board.clone(),
            possible_turns: Vec::new(),
            score: (0.0, 0.0, 0.0),
        };

        let start_depth = std::cmp::min(board.count_pieces(), 16);
        Self::process_node(&mut node, start_depth, start_depth + SEARCH_DEPTH);
        let best_move = opponent_bot.best_move(node).0;
        println!("{:?}, {:?}", best_move, turn);

        // Only second best if opponent did best move
        turn == &best_move
    }
}

impl Bot {
    pub fn new(colour: Colour) -> Self {
        Self { colour }
    }

    // Find the best possible move of possible moves in movenode
    // TODO: maybe best score is greatest difference between black and white win?
    fn best_move(&self, node: MoveNode) -> (Turn, Turn) {
        // Start with best possible being first possible move
        let mut best = &node.possible_turns[0];
        // Also keep track of second best possible for second best
        let mut second_best = best;

        // Keep track of score of (second) best possible move to compare to
        let mut best_score = 0.0;
        let mut second_best_score = 0.0;

        // Get relevant score of move, depending on colour
        let node_score = |score: (f64, f64, f64)| {
            if self.colour == Colour::White {
                score.0
            } else {
                score.1
            }
        };

        // Go over every move
        for i in 0..node.possible_turns.len() {
            let trial = &node.possible_turns[i];

            // A gaming winning endstate is best
            if let MoveResult::End(EndState::Win(colour)) = trial.1 {
                if colour == self.colour {
                    second_best = best;
                    best = trial;

                    second_best_score = best_score;
                    best_score = f64::MAX;
                    // No break because second best might still change
                }
            } else if let MoveResult::Continue(new_node) = &trial.1 {
                let score = node_score(new_node.score);
                if score > best_score {
                    second_best = best;
                    best = trial;

                    second_best_score = best_score;
                    best_score = score;
                } else if score > second_best_score {
                    second_best = trial;
                    second_best_score = score;
                }
            }
        }
        (best.0.clone(), second_best.0.clone())
    }

    // Given a node, calculate its moves and their scores recursively to a certain depth
    fn process_node(node: &mut MoveNode, depth: usize, max_depth: usize) {
        // Assumes white moves first
        let colour = if depth % 2 == 0 {
            Colour::White
        } else {
            Colour::Black
        };
        // Possible turns placing pieces or moving pieces, calulate score using try_turn
        if depth < max_depth && depth < 16 {
            for i in 0..8 {
                let turn = Turn::Place(colour, i);
                if node.board.is_possible_turn(&turn) {
                    Self::try_turn(turn, node, depth, max_depth);
                }
            }
        } else if depth < max_depth && depth >= 16 {
            for i in 0..8 {
                if *node.board.0[i].peek_top() == Piece::Piece(colour) {
                    for j in [1, 4, 7] {
                        let turn = Turn::Move(colour, i, (i + j) % 8);
                        if node.board.is_possible_turn(&turn) {
                            Self::try_turn(turn, node, depth, max_depth);
                        }
                    }
                }
            }
        }
        // Normalise score
        let size: f64 = (node.possible_turns.len() + 1) as f64;
        node.score = (node.score.0 / size, node.score.1 / size, node.score.2 / size);
    }

    fn try_turn(turn: Turn, node: &mut MoveNode, depth: usize, max_depth: usize) {
        // Try the turn on a temp board
        let mut board = node.board.clone();
        board.do_turn(&turn);

        // Calculate score either add relevant won score, or count up scores of subsequent moves
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
                // Recurse on more depth
                let mut new_node = MoveNode {
                    board,
                    possible_turns: Vec::new(),
                    score: (0.0, 0.0, 0.0),
                };
                Self::process_node(&mut new_node, depth + 1, max_depth);
                // After this is processed, add its score to score of current node
                // Might need some reducing of scores
                node.score = (
                    node.score.0 + new_node.score.0,
                    node.score.1 + new_node.score.1,
                    node.score.2 + new_node.score.2,
                );
                node.possible_turns
                    .push((turn, MoveResult::Continue(new_node)));
            }
        }
    }
}
