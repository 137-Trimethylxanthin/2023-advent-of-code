#[cfg(test)]
mod tests {
    use crate::{convert_number, extract_numbers, map_extraction, read_file_to_vec};
    use super::*;

    #[test]
    fn test_extract_numbers() {
        let s = "123 abc 456 def 789";
        let numbers = extract_numbers(s);
        assert_eq!(numbers, vec![123, 456, 789]);
    }

    #[test]
    fn test_convert_number() {
        let map = vec![(10, 20, 30), (40, 50, 60)];
        let num = 25;
        let result = convert_number(num, &map);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_map_extraction() {
        let map = "10 20 30\n40 50 60\n70 80 90";
        let result = map_extraction(map);
        assert_eq!(result, vec![(10, 20, 30), (40, 50, 60), (70, 80, 90)]);
    }
    #[test]
    fn test_main() {
        println!("Hello, world!");
        let v = read_file_to_vec("src/test.txt");
        println!("{:?}", v);
        assert_eq!(v, 46)

    }
}