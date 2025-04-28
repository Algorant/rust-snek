use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;


use crate::draw::draw_block;

// Red, Green, Blue, Opacity
const SNAKE_COLOR: Color = [0.20, 0.50, 0.20, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
#[derive(Debug,Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block {
            x: x + 2,
            y,
        });
        body.push_back(Block {
            x: x + 1,
            y, 
        });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>, width: i32, height: i32) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_position();
        
        // Calculate new position with wrapping
        let (new_x, new_y) = match self.direction {
            Direction::Up => (last_x, if last_y <= 0 { height - 2 } else { last_y - 1 }),
            Direction::Down => (last_x, if last_y >= height - 2 { 1 } else { last_y + 1 }),
            Direction::Left => (if last_x <= 0 { width - 2 } else { last_x - 1 }, last_y),
            Direction::Right => (if last_x >= width - 2 { 1 } else { last_x + 1 }, last_y),
        };
        
        let new_block = Block {
            x: new_x,
            y: new_y,
        };
        
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>, width: i32, height: i32) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        // Calculate next position with wrapping
        match moving_dir {
            Direction::Up => (head_x, if head_y <= 0 { height - 2 } else { head_y - 1 }),
            Direction::Down => (head_x, if head_y >= height - 2 { 1 } else { head_y + 1 }),
            Direction::Left => (if head_x <= 0 { width - 2 } else { head_x - 1 }, head_y),
            Direction::Right => (if head_x >= width - 2 { 1 } else { head_x + 1 }, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;
            if ch == self.body.len() -1 {
                break;
            }
        }
        return false;
    }
}
