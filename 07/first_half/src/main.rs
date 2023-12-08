mod test;

use std::ops::Deref;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, Read};
use regex::Regex;
use std::str::FromStr;


#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}




fn main() {
    println!("Hello, world!");
    let winnings = get_winnings("src/seinsmon.txt");
    println!("winnings: {}", winnings);
}

fn get_winnings(path:&str) -> u64{

    let mut hands: Vec<(HandType, (u32, u32, u32, u32, u32), u32)> = Vec::new();

    let mut file:File = File::open(path).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();

    for line in string.lines() {
        let parts  = line.split(" ").collect::<Vec<&str>>();

        let hand = parts[0];
        let bet = parts[1];

        let numbers = extract_number(bet).unwrap();

        let hand  = score_hand(hand);

        let complete = (hand.0, hand.1, numbers);
        hands.push(complete)
    }




    //println!("{:?}", hands);

    return earnings(hands);

}

fn earnings(mut hands: Vec<(HandType, (u32, u32, u32, u32, u32), u32)>) -> u64 {
    hands.sort_by(|a, b| {
        let a_type = a.0 as u32;
        let b_type = b.0 as u32;
        let a_cards = a.1;
        let b_cards = b.1;


        //i hate my life -_-
        match a_type.cmp(&b_type) {
            std::cmp::Ordering::Equal => {
                match a_cards.0.cmp(&b_cards.0) {
                    std::cmp::Ordering::Equal => match a_cards.1.cmp(&b_cards.1) {
                        std::cmp::Ordering::Equal => match a_cards.2.cmp(&b_cards.2) {
                            std::cmp::Ordering::Equal => match a_cards.3.cmp(&b_cards.3) {
                                std::cmp::Ordering::Equal => a_cards.4.cmp(&b_cards.4),
                                other => other,
                            },
                            other => other,
                        },
                        other => other,
                    },
                    other => other,
                }
            },
            other => other,
        }
    });

    let mut earnings: u64 = 0;
    for (rank, hand) in hands.iter().enumerate() {
        earnings += (rank as u64 + 1) * hand.2 as u64;
    }

    println!("earnings: {:?}", hands);

    earnings
}
fn score_hand(hand: &str, ) -> (HandType, (u32, u32, u32, u32, u32)) {
    use HandType::*;

    let counts = hand.chars().counts();
    let values = counts.values().sorted().join("");
    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => panic!(
            "should never happen. Encountered `{}`",
            value
        ),
    };

    let card_scores = hand
        .chars()
        .map(|card| match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            value => value.to_digit(10).unwrap(),
        })
        .collect_tuple()
        .unwrap();
    (hand_type, card_scores)
}




fn extract_number(s: &str) -> Option<u32> {
    let re = Regex::new(r"\d+").unwrap();
    re.find(s)
        .map(|mat| u32::from_str(mat.as_str()).unwrap())
}

