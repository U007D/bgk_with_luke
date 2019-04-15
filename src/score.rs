use crate::Rolls;

#[cfg(test)]
mod unit_tests;

pub fn score(rolls: &Rolls) -> u16 {
    let mut score = 0;
    let mut prev_score = 0;
    let mut is_prev_spare = false;
    for roll_score in rolls.0.iter().map(|&v| u16::from(v)).collect::<Vec<u16>>() {
        score += roll_score;
        if is_prev_spare {
            score += roll_score;
        }

        if is_spare(roll_score, prev_score) {
            is_prev_spare = true;
        }
        prev_score = roll_score;
    }
    score
}

fn is_spare(v1: u16, v2: u16) -> bool {
    v1 + v2 == 10
}
