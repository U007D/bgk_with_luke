use crate::Rolls;
use std::cell::RefCell;
use std::rc::Rc;

struct Frame {
    score: u16,
}

impl Frame {
    pub fn new() -> Self {
        Frame { score: 0 }
    }
    fn is_spare(&self) -> bool {
        self.score == 10
    }
    fn add_roll(&mut self, points: u16) {
        self.score += points;
    }
}


#[cfg(test)]
mod unit_tests;

pub fn score(rolls: &Rolls) -> u16 {
    // iterate through rolls, 2 balls at a time
    let mut curr_frame = Rc::new(RefCell::new(Frame::new()));
    let mut prev_frame = Rc::new(RefCell::new(Frame::new()));
    let mut frames = rolls
        .0
        .iter()
        .map(|&v| u16::from(v))
    .fold(Vec::new(), |mut frames: Vec<Frame>, v|{
        curr_frame.borrow_mut().add_roll(v);
        if curr_frame.borrow().is_spare() {
            frames.push(curr_frame.replace(Frame::new()));
            prev_frame.replace(curr_frame.replace(Frame::new()));
        }
        frames
    });
    frames.iter().fold(0, |sum, frame| sum + frame.score)

}
