pub fn binary_search(v: &Vec<usize>, t: usize) -> bool {
    let mut min: usize = 0;
    let mut max: usize = v.len();

    while min < max {
        let mid: usize = min + (max - min) / 2;
        if v[mid] == t {
            return true;
        } else if v[mid] > t {
            max = mid;
        } else {
            min = mid + 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let test_vec: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        assert!(binary_search(&test_vec, 0));
        assert!(binary_search(&test_vec, 3));
        assert!(binary_search(&test_vec, 5));
        assert!(!binary_search(&test_vec, 6));
    }

    //TODO Add random tests
}
