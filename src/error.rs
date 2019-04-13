use crate::consts::msg;
use derive_more::*;

#[derive(Debug, Display, From, PartialEq)]
pub enum Error {
    #[display(fmt = "{}: {:?}", "msg::ERR_SAMPLE", "_0")]
    SampleError(std::num::ParseIntError),
}
