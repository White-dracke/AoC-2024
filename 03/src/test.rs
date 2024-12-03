#[cfg(test)]
mod tests {
    use super::super::get_array_from_memory;
    use super::super::get_array_from_memory_conditional;
    use super::super::sum_array_results;

    #[test]
    fn test_get_array_from_memory() {
        let test_file = "test_input.txt";
        let expected_result = [8, 25, 88, 40];
        let result = get_array_from_memory(test_file).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_sum_array_results() {
        let expected_result = 161;
        let result = sum_array_results([8, 25, 88, 40].to_vec()).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_get_array_from_memory_conditional() {
        let test_file = "test_input.txt";
        let expected_result = [8, 40];
        let result = get_array_from_memory_conditional(test_file).unwrap();
        assert_eq!(result, expected_result);
    }
}
