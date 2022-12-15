pub fn sort<T: Ord + Clone>(input: &mut Vec<T>) -> &Vec<T> {
    let mut swapped = false;

    for a in 0..(input.len() - 1) {
        let b = a + 1;

        if &input[a] > &input[b] {
            input.swap(a, b);
            swapped = true;
        }
    }

    if swapped {
        return input;
    } else {
        return sort(input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut i = vec![5, 1, 7, 6];
        let result = sort(&mut i);
        assert_eq!(result, &vec![1, 5, 6, 7]);
    }
}
