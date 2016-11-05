
use std::collections::LinkedList;

use piston_window::Context;
use piston_window::G2d;
use piston_window::rectangle;
use piston_window::types::Color;

use BLOCK_SIZE;

const SNAKE_COLOR: Color = [0.741, 0.765, 0.78, 1.0];

pub enum Direction {
    Up, Down, Left, Right
}

#[derive(Debug, Clone)]
struct Block {
    x: f64,
    y: f64
}

pub struct Snake {
    moving_direction: Direction,
    body: LinkedList<Block>,
    last_removed_block: Option<Block>
}

impl Snake {
    pub fn new(init_x: f64, init_y: f64) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block {
            x: init_x + BLOCK_SIZE * 2.0,
            y: init_y
        });
        body.push_back(Block {
            x: init_x + BLOCK_SIZE,
            y: init_y
        });
        body.push_back(Block {
            x: init_x,
            y: init_y
        });

        Snake {
            moving_direction: Direction::Right,
            body: body,
            last_removed_block: None
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            rectangle(SNAKE_COLOR, [block.x, block.y,
                BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
        }
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        // Change moving direction
        match dir {
            Some(d) => self.moving_direction = d,
            None => {}
        }

        // Retrieve the position of the head block
        let (last_x, last_y): (f64, f64) = self.head_position();

        // The snake moves
        let new_block = match self.moving_direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - BLOCK_SIZE
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + BLOCK_SIZE
            },
            Direction::Left => Block {
                x: last_x - BLOCK_SIZE,
                y: last_y
            },
            Direction::Right => Block {
                x: last_x + BLOCK_SIZE,
                y: last_y
            }
        };
        self.body.push_front(new_block);
        let removed_blk = self.body.pop_back().unwrap();
        self.last_removed_block = Some(removed_blk);
    }

    pub fn head_position(&self) -> (f64, f64) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn restore_last_removed(&mut self) {
        let blk = self.last_removed_block.clone().unwrap();
        self.body.push_back(blk);
    }
}
