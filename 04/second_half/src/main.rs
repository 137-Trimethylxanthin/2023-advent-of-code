use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::scratch_card::ScratchCard;

mod test;
mod scratch_card;


fn main() {
    let timeer = std::time::Instant::now();
    let pile_of_scratchCards = read_scratch_cards_from_file("src/input.txt");
    let mut sum = 0;
    for scratchCard in pile_of_scratchCards.clone() {
        sum += scratchCard.matches;
        println!("Matches: {}", scratchCard.matches)
    }
    println!("Total matches: {}", sum);
    let total_amount = get_total_ammount_of_scratch_cards(pile_of_scratchCards, 0, -1);
    println!("Total amount of scratch cards: {}", total_amount);
    println!("Time: {:?}", timeer.elapsed());
}

fn read_scratch_cards_from_file(path:&str) -> Vec<ScratchCard> {
    let mut scratch_cards:Vec<ScratchCard> = Vec::new();
    let mut file:File = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut scratch = 0.5;
        let part: Vec<&str> = line.split(":").collect();
        let ticket = part[1];
        let part: Vec<&str> = ticket.split("|").collect();
        let winning_num = part[0];
        let my_num = part[1];
        let winning_list: Vec<i32> = get_numbers_from_string(winning_num);
        let my_list: Vec<i32> = get_numbers_from_string(my_num);

        let mut scratch_card = ScratchCard::new(winning_list, my_list);
        scratch_card.scratch();
        scratch_cards.push(scratch_card);
    }
    scratch_cards
}


fn get_numbers_from_string(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect()
}

fn get_total_ammount_of_scratch_cards(scratch_cards:Vec<ScratchCard>, start:i32, mut end: i32) -> i32 {
    let mut total_amount:i32 = 0;
    if start > end {
        end = scratch_cards.len() as i32 - 1;
    }
    //println!("Start: {} | End: {} | {}", start+1, end+1, (end+1) - (start));
    for i in start..(end+1) {

        let matches = scratch_cards[i as usize].matches;
       // println!("Matches: {} | {} | {} | {}", matches, i, scratch_cards.len() -1, i == scratch_cards.len() as i32 -1 );
        if matches != 0 && i != scratch_cards.len() as i32 - 1 {
            total_amount += get_total_ammount_of_scratch_cards(scratch_cards.clone(), i+1, i+matches );
        }
        total_amount += 1;

    }
    println!("Total amount: {}", total_amount);
    total_amount
}