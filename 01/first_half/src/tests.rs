
const WORD_LIST: [&'static str; 4] = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];


#[cfg(test)]
mod tests {
    use crate::{decode_line, decode_list};
    use super::*;
    
    #[test]
    fn first_line(){
        assert_eq!(decode_line(WORD_LIST[0]), 12);
    }

    #[test]
    fn second_line(){
        assert_eq!(decode_line(WORD_LIST[1]), 38);
    }

    #[test]
    fn third_line(){
        assert_eq!(decode_line(WORD_LIST[2]), 15);
    }

    #[test]
    fn fourth_line(){
        assert_eq!(decode_line(WORD_LIST[3]), 77);
    }

    #[test]
    fn decode_my_wordsa(){
        assert_eq!(decode_list(WORD_LIST.iter().map(|&s| s.to_string()).collect()), 142);
    }
}