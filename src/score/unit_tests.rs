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
    // given a single roll
    let rolls = Rolls::new(&[1])?;

    // when `score()` is calculated
    let res = score(&rolls);

    // then the result should be 1
    assert_eq!(1, res);

    Ok(())
}

#[test]
fn score_all_one_balls_returns_20() -> Result<()> {
    // given a single roll
    let rolls = Rolls::new(&[1; 20])?;

    // when `score()` is calculated
    let res = score(&rolls);

    // then the result should be 1
    assert_eq!(20, res);

    Ok(())
}

#[test]
fn score_spare_score_1_returns_12() -> Result<()> {
    // given a roll with a strike
    let rolls = Rolls::new(&[5, 5, 1])?;

    // when `score()` is calculated
    let res = score(&rolls);

    // then the result should be 12
    assert_eq!(12, res);

    Ok(())
}


#[test]
fn score_open_then_spare_then_1_returns_14() -> Result<()> {
    // given a roll with a strike
    let rolls = Rolls::new(&[3, 5, 5, 1])?;

    // when `score()` is calculated
    let res = score(&rolls);

    // then the result should be 12
    assert_eq!(14, res);

    Ok(())
}

#[test]
fn score_all_spares_returns_() -> Result<()> {
    // given a roll with a strike
    let rolls = Rolls::new(&[5; 21])?;

    // when `score()` is calculated
    let res = score(&rolls);

    // then the result should be 12
    assert_eq!(150, res);

    Ok(())
}
