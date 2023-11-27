pub fn two_crystal_balls(v: Vec<bool>) -> Option<usize> {
    let n: usize = v.len();
    let jump_amount: usize = (n as f64).sqrt() as usize;

    let mut i: usize = jump_amount;
    while i < n && !v[i] {
        i += jump_amount;
    }

    let start: usize = if i >= jump_amount { i - jump_amount } else { 0 };
    for j in start..i.min(n) {
        if v[j] {
            return Some(j);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_crystal_balls() {
        let test_vec: Vec<bool> = vec![false, false, false, false, false, true, true];
        assert_eq!(two_crystal_balls(test_vec), Some(5));
    }
}
