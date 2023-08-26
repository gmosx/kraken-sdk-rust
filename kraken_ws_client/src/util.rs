use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

// #todo make this customizable.
pub fn gen_next_id() -> u64 {
    rand::random()
}
