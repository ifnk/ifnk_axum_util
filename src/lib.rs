use crate::util_entity::response::json_success;

mod util;
mod util_entity;
mod middleware;

pub use crate::util::*;
pub use crate::util_entity::*;
pub use crate::middleware::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
