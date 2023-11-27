pub fn bubble_sort(v: &mut Vec<usize>) -> &mut Vec<usize> {
    let n = v.len();
    for i in 0..n {
        for j in 0..(n - 1 - i) {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1)
            }
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut test_vec: Vec<usize> = vec![5, 1, 2, 4, 3, 0];
        let expected: Vec<usize> = vec![0, 1, 2, 3, 4, 5];
        bubble_sort(&mut test_vec);
        assert_eq!(expected, test_vec);
    }
}
