pub fn hello() {
    println!("Hello, world!");
}

pub fn validate() {
    println!("Validating...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        hello();
    }

    #[test]
    fn test_validate() {
        validate();
    }
}
