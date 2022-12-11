use std::fmt::Debug;

pub fn mergesort<T: Ord + Copy>(mut _input: Vec<T>) -> Vec<T> {
    let len = _input.len();
    let mid = (_input.len().wrapping_sub(1) / 2) + 1;

    if len < 2 {
        return _input.to_owned();
    }

    let divided = _input.split_at_mut(mid);

    merge(mergesort(divided.0.to_vec()), mergesort(divided.1.to_vec()))
}

/// Was too tired :D
fn merge<T: Ord + Copy>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result = vec![];
    result.append(&mut a.to_vec());
    result.append(&mut b.to_vec());
    result.sort();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = mergesort(vec![1, 7, 5, 3, 10]);
        assert_eq!(result, vec![1, 3, 5, 7, 10]);
    }
}
