use crate::Rolls;

#[cfg(test)]
mod unit_tests;

pub fn score(rolls: &Rolls) -> u16 {
    foo(rolls.0.iter(), 1)
}

// variable-width fold
// Item is an associated type
fn foo<'a>(mut rolls: impl Iterator<Item=&'a u8> + Clone, frame_number: usize) -> u16 {
    // base case:
    if frame_number > 10 {
        return 0;
    }

    // stuff!
    // by_ref = by_ref_mut
    // helps us keep ownership by giving exclusive unique access:
    // unique access vs ownership!!!
    let frame_score = rolls.by_ref().take(2).sum::<u8>();
    // take drops the borrow here.
    let mut bonus = 0;
    if frame_score == 10 {
        bonus = rolls.clone().take(1).sum::<u8>();
    }

    // recusive case - tail recursion
    // rolls still owns the data
    u16::from(frame_score + bonus) + foo(rolls, frame_number + 1)
}
