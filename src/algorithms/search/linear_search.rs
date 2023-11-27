pub fn linear_search(v: &Vec<usize>, t: usize) -> bool {
    for n in v.iter() {
        if *n == t {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let test_vec: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        assert!(linear_search(&test_vec, 0));
        assert!(linear_search(&test_vec, 3));
        assert!(linear_search(&test_vec, 5));
        assert!(!linear_search(&test_vec, 6));
    }
}
