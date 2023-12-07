use std::io::Error;
use std::io::prelude::BufRead;
use utils::file::read_file;
use adventofcode_2023day_04::card;

pub fn start() -> Result<(), Error> {
    let reader = read_file("solutions/day_04/input.txt").expect("Err");
    let mut result: u16 = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let card = card::Card::from(line.as_str());
                println!("{card:?}");
                result += card.value;
            },
            Err(error) => panic!("Problem opening the file: {:?}", error)
        }
    }

    println!("\n\n\nAdvent Of Code - 2023 | Day 04 | 'Scratchcards': Part 1 \n Answer: {}", result);
    Ok(())
}


#[cfg(test)]
mod test {
    use test_case::test_case;
    use adventofcode_2023day_04::card;

    const TEST_CASE_1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    const TEST_CASE_2: &str = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
    const TEST_CASE_3: &str = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
    const TEST_CASE_4: &str = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
    const TEST_CASE_5: &str = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
    const TEST_CASE_6: &str = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";


    #[test_case(TEST_CASE_1 => 8)]
    #[test_case(TEST_CASE_2 => 2)]
    #[test_case(TEST_CASE_3 => 2)]
    #[test_case(TEST_CASE_4 => 1)]
    #[test_case(TEST_CASE_5 => 0)]
    #[test_case(TEST_CASE_6 => 0)]
    fn process_card_string_indivisual(test_case: &str) -> u16 {
        let card = card::Card::from(test_case);
        card.value
    }

    #[test_case([TEST_CASE_1, TEST_CASE_2, TEST_CASE_3, TEST_CASE_4, TEST_CASE_5, TEST_CASE_6].join("\n") => 13)]
    fn process_card_string_group(test_case: String) -> u16 {
        let mut result = 0;
        for line in test_case.lines() {
            let card = card::Card::from(line);
            result += card.value;
        }
        result
    }






    
}