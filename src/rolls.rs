use crate::Result;
use std::borrow::Borrow;

pub struct Rolls(pub(crate) Vec<u8>);

impl Rolls {
    pub fn new<I>(rolls: I) -> Result<Self> where I: IntoIterator,
                                                  I::Item: Borrow<u8>, {
        Ok(Rolls(rolls.into_iter()
                      .map(|v| *v.borrow())
                      .collect()))
    }
}
