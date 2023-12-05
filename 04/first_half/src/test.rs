#[cfg(test)]
mod tests {
    use crate::{get_numbers_from_string, solve_file};
    use super::*;

    #[test]
    fn test_get_numbers_from_string() {
        let input = "41 48 83 86 17";
        let expected = vec![41, 48, 83, 86, 17];
        assert_eq!(get_numbers_from_string(input), expected);
    }

    #[test]
    fn test_solve_file() {
        let input = "src/test.txt";
        let expected = 2;
        assert_eq!(solve_file(input), 13);
    }
}