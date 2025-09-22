use std::collections::VecDeque;

use crate::{entities::types::Direction, world::types::GridPos};

/// The main entity in the game,
/// which is stored on both the
/// client and server sides with the same structure.
pub struct Snake {
    /// A variable that is necessary in many cases,
    /// indicating the direction in which our snake is moving.
    ///
    /// 1. To add a tail when eat an apple
    /// 2. To remove the tail
    /// 3. Prediction??
    pub direction: Direction,

    pub body: VecDeque<GridPos>,
}

impl Snake {
    pub fn new() -> Snake {
        // TODO: randomize direction???

        Snake {
            direction: Direction::North, // TODO: test variant
            body: VecDeque::new(),
        }
    }

    /// Absolutely genius and simple function which
    /// Have a O(1) time for operation, and do ALL
    /// logical of movement for game tick
    pub fn move_forward(&mut self) {
        let head = self.body.front().expect("Snake has no head!");

        let new_head = match self.direction {
            Direction::North => GridPos {
                x: head.x,
                y: head.y - 1,
            },
            Direction::South => GridPos {
                x: head.x,
                y: head.y + 1,
            },
            Direction::East => GridPos {
                x: head.x + 1,
                y: head.y,
            },
            Direction::West => GridPos {
                x: head.x - 1,
                y: head.y,
            },
        };

        self.body.push_front(new_head);

        self.body.pop_back();
    }
}
