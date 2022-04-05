//! Utilities to work with the fibonacci sequence.

pub fn fibonacci(n: usize) -> Vec<f64> {
    let mut x = vec![1.0, 1.0];

    for i in 2..n {
        let next_x = x[i - 1] + x[i - 2];
        x.push(next_x);
    }

    x
}

#[cfg(test)]
mod tests {
    #[test]
    fn fibonacci() {
        let result = super::fibonacci(10);
        let expected = vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0, 55.0];

        assert_eq!(result, expected);
    }
}
