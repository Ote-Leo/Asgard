pub fn add_two(num: i32) -> i32 {
    num + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        for n in 1..100 {
            assert_eq!(n, add_two(n - 2))
        }
    }
}
