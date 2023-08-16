pub fn add_one(num: i32) -> i32 {
    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for n in 1..100 {
            assert_eq!(n, add_one(n - 1))
        }
    }
}