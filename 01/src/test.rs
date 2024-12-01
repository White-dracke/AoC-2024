#[cfg(test)]
mod tests {
    use super::super::calculate_total_distance;
    use super::super::calculate_total_score;

    #[test]
    fn test_calculate_total_distance() {
        let test_file = "test_input.txt";
        let expected_distance = 11;
        let result = calculate_total_distance(test_file).unwrap();
        assert_eq!(result, expected_distance);
    }

    #[test]
    fn test_calculate_similarity_score() {
        let test_file = "test_input.txt";
        let expected_score = 31;
        let result = calculate_total_score(test_file).unwrap();
        assert_eq!(result, expected_score);
    }
}
