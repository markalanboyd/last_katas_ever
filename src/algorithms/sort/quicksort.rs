fn qs_recurse(v: &mut Vec<usize>, lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let pivot_idx: usize = partition(v, lo, hi);

    if pivot_idx > 0 {
        qs_recurse(v, lo, pivot_idx - 1);
    }
    if pivot_idx + 1 < hi {
        qs_recurse(v, pivot_idx + 1, hi);
    }
}

fn partition(v: &mut Vec<usize>, lo: usize, hi: usize) -> usize {
    let pivot: usize = v[hi];
    let mut i: usize = lo;

    for j in lo..hi {
        if v[j] <= pivot {
            v.swap(i, j);
            i += 1;
        }
    }

    v.swap(i, hi);
    return i;
}

pub fn quicksort(v: &mut Vec<usize>) {
    qs_recurse(v, 0, v.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let mut test_vec: Vec<usize> = vec![4, 1, 5, 2, 3];
        let expected: Vec<usize> = vec![1, 2, 3, 4, 5];
        quicksort(&mut test_vec);
        assert_eq!(test_vec, expected)
    }
}
