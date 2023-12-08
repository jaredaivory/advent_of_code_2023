use std::io::Error;
use std::io::prelude::BufRead;
use utils::file::read_file;
use adventofcode_2023day_04::card::{self, Card};


pub fn start() -> Result<(), Error> {
    let reader = read_file("solutions/day_04/input.txt").expect("Err");
    let mut result: u16 = 0;

    let card_strings: Vec<String>  = reader.lines().map(|x| x.unwrap().into()).collect();
    let mut cards: Vec<Card> = vec![];

    for card_string in card_strings {
        cards.push(Card::from(card_string.as_str()));
    }

    for i in 0..cards.len(){
        let count = cards[i].count;
        let mut j = 0;
        while (j as u16) <= count && j + i + 1 < cards.len() {
            cards[j+i+1].copies += 1;
            j+=1;
        }
        println!("{}", cards[i].copies);
        result += cards[i].copies;
    }

    

    println!("\n\n\nAdvent Of Code - 2023 | Day 04 | 'Scratchcards': Part 1 \n Answer: {}", result);
    Ok(())
}
