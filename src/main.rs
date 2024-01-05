mod basic;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use crate::basic;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_add() {
        basic::add()
    }
}
