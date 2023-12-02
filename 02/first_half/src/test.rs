#[cfg(test)]
mod tests {
    use crate::{ get_sum_of_vec, read_file_to_bags};

    #[test]
    fn play_game_t1() {
        let games = read_file_to_bags("src/testGame.txt",12, 13, 14);

        let sum = get_sum_of_vec(games);
        assert_eq!(sum, 8)
    }
}