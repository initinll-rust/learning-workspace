use rand;
use lib_common::common;

pub fn add_one(num: i32) -> i32 {
    common("add_one crate");
    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
