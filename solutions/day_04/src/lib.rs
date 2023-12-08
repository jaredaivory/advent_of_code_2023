pub mod card {
    use std::{collections::{HashSet}, io::Error};
    
    #[derive(Debug, Default, Clone)]
    pub struct Card {
        pub card_id: u16,
        pub numbers: HashSet<String>,
        pub winning_numbers: HashSet<String>,
        pub value: u16,
        pub copies: u16,
        pub count: u16,
    }

    fn create_card_from_str(card_string:  &str) -> Result<Card, Error> {
        let card_split: Vec<String> = card_string.split(":").map(|s| s.into()).collect();

        let card_id: u16 = card_split[0].split(" ").last().unwrap_or("0").parse().unwrap();
        let numbers: Vec<Vec<String>> = card_split[1]
                            .split("|")
                            .map (|numbers| {
                                numbers.trim()
                                    .split_whitespace()
                                    .map(|s| s.into())
                                    .collect::<Vec<String>>()
                            }).collect();
            
        let numbers_set: HashSet<String> = HashSet::from_iter(numbers[0].iter().cloned());
        let winning_number_set: HashSet<String>  = HashSet::from_iter(numbers[1].iter().cloned());
        
        let mut card_value = 0;
        let matching_numbers = numbers_set.intersection(&winning_number_set).count() as u32;
        if matching_numbers != 0 {
            card_value = u16::pow(2,matching_numbers - 1);
        }
        
        
        Ok(Card {
            card_id,
            numbers: numbers_set,
            winning_numbers: winning_number_set,
            value: card_value,
            copies: 1,
            count: matching_numbers as u16
        })
    }

    impl<'a> From<&'a str> for Card {
        fn from(value: &'a str) -> Self {
            create_card_from_str(value).unwrap()
        }
    }
}

