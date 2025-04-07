mod array;
mod string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::main;

    #[test]
    fn test_main() {
        assert!(main().is_ok());
    }
}
