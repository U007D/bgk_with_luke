use crate::Rolls;

#[cfg(test)]
mod unit_tests;

pub fn score(rolls: &Rolls) -> u16 {
    rolls.0.iter()
           .map(|&v| u16::from(v))
           .sum()
}