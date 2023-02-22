#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_something() {
        let result = do_something();
        assert_eq!(result, 1);
    }
}


pub fn do_something() -> i32 {
    return 1;
}
