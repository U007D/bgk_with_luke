use crate::Rolls;

#[cfg(test)]
mod unit_tests;

pub fn score(rolls: &Rolls) -> u16 {
    foo(rolls.0.iter(), 1)
}

// variable-width fold
// Item is an associated type
fn foo<'a>(mut rolls: impl Iterator<Item = &'a u8> + Clone, frame_number: usize) -> u16 {
    // base case:
    if frame_number > 10 {
        return 0;
    }

    let mut frame_score = 0;
    let roll_count = rolls
        // by_ref ("by_ref_mut") helps us keep ownership by giving
        // exclusive "unique access" via mutable borrowing -
        // (different than transferring ownership)
        .by_ref()
        .enumerate()
        // double ref here with take_while: Because the closure passed
        // to take_while() takes a reference, and many iterators
        // iterate over references, this leads to a possibly confusing
        // situation, where the type of the closure is a double
        // reference:
        .take_while(|&(i, &roll)| {
            // This is our "predicate" - a single variable function that returns a bool
            frame_score += roll;
            // take_while will throw away the item if predicate returns false!!!
            i == 0 && roll < 10
        }).count()
        // +1 accounts for the last take_while iteration, which is
        // dropped when the predicate returns false.
        + 1;
    // take_while drops the borrow here.

    let bonus = rolls
        .clone()
        .take(match (roll_count, frame_score) {
            (1, 10) => 2,
            (2, 10) => 1,
            _ => 0,
        })
        .sum::<u8>();

    // recusive case - tail recursion
    // rolls still owns the data
    u16::from(frame_score + bonus) + foo(rolls, frame_number + 1)
}
