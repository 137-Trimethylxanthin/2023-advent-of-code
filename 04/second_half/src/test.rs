#[cfg(test)]
mod tests {
    use crate::{get_numbers_from_string, get_total_ammount_of_scratch_cards, read_scratch_cards_from_file};
    use crate::scratch_card::ScratchCard;
    use super::*;

    #[test]
    fn test_get_numbers_from_string() {
        let s = "1 2 3 4 5";
        let result = get_numbers_from_string(s);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_read_scratch_cards_from_file() {
        let path = "src/test.txt";
        let result = read_scratch_cards_from_file(path);
        // Here you should replace `expected_result` with the expected output of the function
        let expected_result = vec![ScratchCard::new(vec![1, 2, 3], vec![1, 2, 3])];
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_get_total_ammount_of_scratch_cards() {
        let path = "src/test.txt";
        let scratch_cards = read_scratch_cards_from_file(path);
        let result = get_total_ammount_of_scratch_cards(scratch_cards, 0, -1);
        assert_eq!(result, 30);
    }

}