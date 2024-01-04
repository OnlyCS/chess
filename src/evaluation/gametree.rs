use std::collections::HashMap;

use crate::prelude::*;

#[derive(Clone, Default)]
pub struct GameTree {
    pub id: u64,
    pub depth: u8,
    pub position: Position,
    pub children: HashMap<Position, GameTree>,
    pub eval: f64,
}

impl GameTree {
    pub fn new(position: Position, depth: u8) -> Self {
        Self {
            id: rng::dense_random(),
            depth,
            position,
            children: HashMap::new(),
            eval: 0.0,
        }
    }

    pub fn populate_children(&mut self, depth: u8) {
        if !self.children.is_empty() {
            for child in self.children.values_mut() {
                child.populate_children(depth - 1);
            }

            return;
        }

        if depth == 0 {
            return;
        }

        for from_sq in self.position.filled().bit_pos_iter() {
            for to_sq in self.position.moves_of(from_sq).bit_pos_iter() {
                let mut child = self.position;
                child.make_move(from_sq, to_sq);

                let mut child_node = GameTree::new(child, depth);
                child_node.populate_children(depth - 1);

                self.children.insert(child, child_node);
            }
        }
    }

    pub fn prune(&mut self, position: Position) -> Option<GameTree> {
        self.children.remove(&position)
    }

    pub fn move_into(&mut self, position: Position) {
        if let Some(pos) = self.prune(position) {
            *self = pos;
        }
    }

    pub fn minimax(
        &mut self,
        depth: u8,
        mut alpha: f64,
        mut beta: f64,
        maximizing_player: bool,
        evaluator: impl StaticEvaluator,
    ) {
        if depth == 0 {
            self.eval = evaluator.eval(&self.position);
            return;
        }

        let mut eval = if maximizing_player {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };

        let mut prune = vec![];

        for (position, child) in &mut self.children {
            child.minimax(depth - 1, alpha, beta, !maximizing_player, evaluator);

            if maximizing_player {
                eval = eval.max(child.eval);
                alpha = alpha.max(eval);
            } else {
                eval = eval.min(child.eval);
                beta = beta.min(eval);
            }

            if beta <= alpha {
                prune.push(*position);
                break;
            }
        }

        for id in prune {
            self.prune(id);
        }

        self.eval = eval;
    }
}
