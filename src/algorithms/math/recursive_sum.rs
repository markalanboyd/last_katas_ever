pub fn recursive_sum(n: usize) -> Option<usize> {
    match n {
        0 => None,
        1 => Some(1),
        _ => match recursive_sum(n - 1) {
            Some(sum) => Some(sum + n),
            None => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_sum() {
        assert_eq!(recursive_sum(5), Some(15));
    }

    #[test]
    fn test_recursive_sum_zero() {
        assert_eq!(recursive_sum(0), None);
    }
}
