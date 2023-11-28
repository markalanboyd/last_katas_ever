pub fn recursive_sum(int: usize) -> usize {
    if int == 1 {
        return 1;
    }

    return int + recursive_sum(int - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_sum() {
        assert_eq!(recursive_sum(5), 15);
    }
}
