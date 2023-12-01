

#[cfg(test)]
mod tests {
    use crate::{decode_line, replace_numeric_words_with_ints, decode_list};

    const WORD_LIST:[&str; 7] = ["two1nine",
                                     "eightwothree",
                                     "abcone2threexyz",
                                     "xtwone3four",
                                     "4nineeightseven2",
                                     "zoneight234",
                                     "7pqrstsixteen"];

    #[test]
    fn first_line() {
        let replaced_word = replace_numeric_words_with_ints(WORD_LIST[0].into());
        assert_eq!(decode_line((&replaced_word).into()), 29, "Line 1 did not decode correctly")
    }

    #[test]
    fn second_line() {
        let replaced_word = replace_numeric_words_with_ints(WORD_LIST[1].into());
        assert_eq!(decode_line((&replaced_word).into()), 83, "Line 2 did not decode correctly");
    }

    #[test]
    fn third_line() {
        let replaced_word = replace_numeric_words_with_ints(WORD_LIST[2].into());
        assert_eq!(decode_line((&replaced_word).into()), 13, "Line 3 did not decode correctly");
    }

    #[test]
    fn fourth_line() {
        let replaced_word = replace_numeric_words_with_ints(WORD_LIST[3].into());
        assert_eq!(decode_line((&replaced_word).into()), 24, "Line 4 did not decode correctly");
    }

    #[test]
    fn fifth_line() {
        let replaced_word = replace_numeric_words_with_ints(WORD_LIST[4].into());
        assert_eq!(decode_line((&replaced_word).into()), 42, "Line 5 did not decode correctly");
    }

    #[test]
    fn sixth_line() {
        let replaced_word = replace_numeric_words_with_ints(WORD_LIST[5].into());
        assert_eq!(decode_line((&replaced_word).into()), 14, "Line 6 did not decode correctly");
    }

    #[test]
    fn seventh_line() {
        let replaced_word = replace_numeric_words_with_ints(WORD_LIST[6].into());
        assert_eq!(decode_line((&replaced_word).into()), 76, "Line 7 did not decode correctly");
    }

    #[test]
    fn decode_vec(){
        let mut replaced_vec = Vec::new();

        for line in WORD_LIST{
            replaced_vec.push(replace_numeric_words_with_ints(line.into()))
        }

        assert_eq!(decode_list(replaced_vec),281);
    }

}