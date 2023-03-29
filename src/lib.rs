#[allow(non_snake_case)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod api_types;
pub mod client;
pub mod managers;
pub mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
