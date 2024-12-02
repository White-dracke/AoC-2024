#[cfg(test)]
mod tests {
    use super::super::count_safe_results;
    use super::super::count_safe_results_with_tolerance;

    #[test]
    fn test_count_safe_results() {
        let test_file = "test_input.txt";
        let expected_result = 2;
        let result = count_safe_results(test_file).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_count_safe_results_with_tolerance() {
        let test_file = "test_input.txt";
        let expected_result = 3;
        let result = count_safe_results_with_tolerance(test_file).unwrap();
        assert_eq!(result, expected_result);
    }
}
