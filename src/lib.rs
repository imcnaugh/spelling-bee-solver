
fn _hello() -> String {
    String::from("Hello, world!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_test() {
        assert_eq!(_hello(), "Hello, world!".to_string());
    }
}