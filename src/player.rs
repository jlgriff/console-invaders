use std::time::Duration;
use crate::{NUM_COLS, NUM_ROWS, NUM_SHOTS};
use crate::frame::{Drawable, Frame};
use crate::laser::Laser;

pub struct Player {
    x: usize,
    y: usize,
    lasers: Vec<Laser>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            lasers: Vec::new(),
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
    pub fn shoot(&mut self) -> bool {
        if self.lasers.len() < NUM_SHOTS {
            self.lasers.push(Laser::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }
    pub fn update(&mut self, delta: Duration) {
        for laser in self.lasers.iter_mut() {
            laser.update(delta);
        }
        self.lasers.retain(|laser| !laser.dead());
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
        for laser in self.lasers.iter() {
            laser.draw(frame);
        }
    }
}
