use super::*;
use crate::Result;
#[test]
fn score_no_rolls_returns_0() -> Result<()> {
    // given an empty `Rolls`
    let rolls = Rolls::new(&[])?;

    // when `score()` is calculated
    let res = score(rolls);

    // then the result should be 0
    assert_eq!(0, res);

    Ok(())
}
