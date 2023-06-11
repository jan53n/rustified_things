fn magnitude(v: &Vec<i64>) -> f64 {
    let k: i64 = v.iter().map(|x| x.pow(2)).sum();
    f64::sqrt(k as f64)
}

fn dot_product(left: &Vec<i64>, right: &Vec<i64>) -> i64 {
    left.iter().zip(right.iter()).map(|(x, y)| x * y).sum()
}

// find cosine similarity between two vectors
pub fn cosine_similarity(left: Vec<i64>, right: Vec<i64>) -> f64 {
    let dot = dot_product(&left, &right) as f64;
    let cross_product = magnitude(&left) * magnitude(&right);
    let result = dot / cross_product;

    if result.is_nan() {
        return 0.0;
    }

    (result * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = cosine_similarity([3, 2, 0, 5].to_vec(), [1, 0, 0, 0].to_vec());
        assert_eq!(result, 0.49);
    }

    #[test]
    fn it_eqalls() {
        let result = cosine_similarity([1, 0, 0, 0].to_vec(), [1, 0, 0, 0].to_vec());
        assert_eq!(result, 1.0);
    }

    #[test]
    fn no_similarity() {
        let result = cosine_similarity([0, 0, 0, 0].to_vec(), [9, 9, 9, 9].to_vec());
        assert_eq!(result, 0.0);
    }
}
