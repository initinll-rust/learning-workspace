use lib_common::common;

pub fn add_two(num: i32) -> i32 {
    common("add_two crate");
    num + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
