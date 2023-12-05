#[cfg(test)]
mod tests {
    use crate::{convert_number, extract_numbers, map_extraction, read_file_to_vec};
    use super::*;

    #[test]
    fn test_extract_numbers() {
        let input = "123 456 789";
        let expected = vec![123, 456, 789];
        assert_eq!(extract_numbers(input), expected);
    }

    #[test]
    fn test_convert_number() {
        let map = vec![(10, 20, 30), (40, 50, 60)];
        assert_eq!(convert_number(25, &map), 15);
        assert_eq!(convert_number(55, &map), 45);
        assert_eq!(convert_number(65, &map), 55);
    }

    #[test]
    fn test_map_extraction() {
        let map = "10 20 30\n40 50 60";
        let expected = vec![(10, 20, 30), (40, 50, 60)];
        assert_eq!(map_extraction(map), expected);
    }
    #[test]
    fn test_main() {
        let v = read_file_to_vec("src/test.txt");
        assert_eq!(v, 35);
    }
}