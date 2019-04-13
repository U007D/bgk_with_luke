use super::*;
use crate::{Result, Rolls};
#[test]
fn score_no_rolls_returns_0() -> Result<()> {
    // given an empty `Rolls`
    let rolls = Rolls::new(&[])?;

    // when `score()` is calculated
    let res = score(&rolls);

    // then the result should be 0
    assert_eq!(0, res);

    Ok(())
}

#[test]
fn create_rolls_from_vec() -> Result<()> {
    // given an empty `Rolls`
    let rolls = Rolls::new(Vec::<u8>::new())?;

    // when `score()` is calculated
    let res = score(&rolls);

    // then the result should be 0
    assert_eq!(0, res);

    Ok(())
}

#[test]
fn score_gutterball_returns_0() -> Result<()> {
    // given an empty `Rolls`
    let rolls = Rolls::new(&[0])?;

    // when `score()` is calculated
    let res = score(&rolls);

    // then the result should be 0
    assert_eq!(0, res);

    Ok(())
}

#[test]
fn score_all_gutterballs_returns_0() -> Result<()> {
    // given an empty `Rolls`
    let rolls = Rolls::new(&[0; 20])?;

    // when `score()` is calculated
    let res = score(&rolls);

    // then the result should be 0
    assert_eq!(0, res);

    Ok(())
}

#[test]
fn score_one_ball_returns_1() -> Result<()> {
    // given an empty `Rolls`
    let rolls = Rolls::new(&[1])?;

    // when `score()` is calculated
    let res = score(&rolls);

    // then the result should be 0
    assert_eq!(1, res);

    Ok(())
}

