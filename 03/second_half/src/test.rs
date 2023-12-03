#[cfg(test)]
mod tests {
    use crate::{combine_numbers, read_file_to_vec,get_gear_numbers_of_a_vec};

    #[test]
    fn it_works() {
        let blueprint = read_file_to_vec("src/test.txt");
        let sum = combine_numbers(blueprint);
        let gear_number = get_gear_numbers_of_a_vec(sum);
        assert_eq!(gear_number, 467835);
    }

}