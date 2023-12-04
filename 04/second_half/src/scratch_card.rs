#[derive(Clone)]
pub struct ScratchCard {
    winning_numbers:Vec<i32>,
    my_numbers:Vec<i32>,
    pub(crate) matches:i32,
}

impl ScratchCard {
    pub(crate) fn new(winning_numbers:Vec<i32>, my_numbers:Vec<i32>) -> ScratchCard {
        ScratchCard {
            winning_numbers,
            my_numbers,
            matches:0,
        }
    }

    pub(crate) fn scratch(&mut self) {
        for my_number in &self.my_numbers {
            for winning_number in &self.winning_numbers {
                if my_number == winning_number {
                    self.matches += 1;
                }
            }
        }
    }
}