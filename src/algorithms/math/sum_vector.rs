pub fn sum_vector(v: &Vec<usize>) -> usize {
    let mut sum: usize = 0;
    for n in v.iter() {
        sum += n;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_vector() {
        let test_vec: Vec<usize> = vec![1, 2, 3, 4, 5];
        let expected: usize = 15;
        let result = sum_vector(&test_vec);
        assert_eq!(expected, result);
    }
}
