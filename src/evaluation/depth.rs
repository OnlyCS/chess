use std::sync::Arc;

use atomicfloat::AtomicF64;

use crate::prelude::*;

#[derive(PartialEq)]
pub struct GameTreeNode {
    pub position: Position,
    pub children: Vec<GameTreeNode>,
}

impl GameTreeNode {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            children: vec![],
        }
    }

    pub fn populate(&mut self, depth: usize) {
        let position = *&self.position;

        if depth == 0 {
            return;
        }

        if self.children.is_empty() {
            self.children = self
                .position
                .pieces_of_turn()
                .bit_pos_iter()
                .flat_map(|from| {
                    self.position.moves_of(from).bit_pos_iter().map(move |to| {
                        let mut child = position;
                        child.make_move(from, to);

                        child
                    })
                })
                .map(|pos| GameTreeNode::new(pos))
                .collect();
        }

        self.children
            .iter_mut()
            .par_bridge()
            .for_each(|n| n.populate(depth - 1))
    }

    pub fn leaves<'a>(&'a mut self, collect: &mut Vec<&'a mut GameTreeNode>) {
        if self.children.is_empty() {
            collect.push(self);
        } else {
            for child in &mut self.children {
                child.leaves(collect);
            }
        }
    }

    fn replace(&mut self, idx: usize) {
        *self = self.children.swap_remove(idx);
    }
}

pub struct GameTreeRoot {
    pub actual_root: GameTreeNode,
    pub eval: Arc<AtomicF64>,
}

impl GameTreeRoot {
    pub fn new(position: Position, eval_link: Arc<AtomicF64>) -> Self {
        Self {
            actual_root: GameTreeNode::new(position),
            eval: eval_link,
        }
    }

    pub fn populate(&mut self, depth: usize) {
        self.actual_root.populate(depth)
    }

    pub fn move_into(&mut self, from: Square, to: Square) {
        let move_made = {
            let mut p = *&self.actual_root.position;
            p.make_move(from, to);

            p
        };

        for i in 0..self.actual_root.children.len() {
            let position = self.actual_root.children[i].position;

            if position == move_made {
                self.actual_root.replace(i);
            }
        }

        panic!("move not found in children");
    }

    pub fn leaves<'a>(&'a mut self, collect: &mut Vec<&'a mut GameTreeNode>) {
        self.actual_root.leaves(collect);
    }
}
