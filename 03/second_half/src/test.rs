#[cfg(test)]
mod tests {
    use crate::{find_parts_numbers, read_file_to_vec, sum_of_vec};

    #[test]
    fn it_works() {
        let blueprint = read_file_to_vec("src/test.txt");
        let numbers = find_parts_numbers(blueprint);
        let sum = sum_of_vec(numbers);
        assert_eq!(sum, 4361)
    }
}