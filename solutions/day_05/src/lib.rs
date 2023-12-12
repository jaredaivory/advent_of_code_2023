
pub mod solution {
    use std::{collections::HashMap};
    use std::error::Error;

    use rangemap::RangeMap;

    #[derive(Debug)]
    pub struct Solution {
        seeds: Vec<u128>,
        collection: HashMap<String, MyRangeMap>
    }

    impl Solution {
        pub fn get_value(&self, seed: u128) -> Result<u128, ()>{
            let mut key_type: String = String::from("seed");
            let mut key = seed;
            let mut value =  0;
            while self.collection.contains_key(&key_type) {
                let rangemap = self.collection.get(&key_type).unwrap();
                value = rangemap.get(key);
                key = value;
                key_type = rangemap.value.to_string();
            }
            Ok(value)
        }

        pub fn solve(&self) -> u128 {
            let mut min: u128 = u128::MAX;
            for seed in &self.seeds {
                min = std::cmp::min(self.get_value(*seed).unwrap(), min);
            }
            min
        }


        pub fn create(s: &str) ->  Self {
            let vec_input_components: Vec<&str> =  s.split("\n\r\n").filter(|x| !x.is_empty()).collect();
            dbg!("{:?}", &vec_input_components);

            let seeds: Vec<u128> = vec_input_components[0].split(":").last().unwrap()
                .trim()
                .split(" ")
                .map(|x|{
                    x.parse::<u128>().unwrap()
                }).collect();
            
            let mut collection: HashMap<String, MyRangeMap> = HashMap::new();
            
            for str_key_to_value in &vec_input_components[1..] {
                let rangemap = MyRangeMap::from(*str_key_to_value);
                collection.insert(rangemap.key.to_string(), rangemap);
            }

            Solution {
                seeds,
                collection
            }
        }
    }

    #[derive(Debug)]
    pub struct MyRangeMap {
        key: String,
        value: String,
        pub rangemap: RangeMap<u128, u128>
    }

    impl MyRangeMap {
        /// Returns the calculated location of the corresponding map
        /// to the given key, if the key is covered by any range in the map. Else: given key
        pub fn get(&self, key: u128) -> u128 {
            match self.rangemap.get_key_value(&key) {
                Some((range, value)) => key - range.start + value,
                None => key,
            }
        }
    }
    

    fn str_to_rangemap(s: &str) -> MyRangeMap {
        // seed-to-soil map:
        // 50 98 2
        // 52 50 48
        let str_rangemap = s.split(":").collect::<Vec<&str>>();

        // Definition of what the mapping represents
        let key_value = str_rangemap[0].split("-to-").collect::<Vec<&str>>();
  
        
        let key = key_value[0].to_string();
        let value = key_value[1].split_whitespace().collect::<Vec<&str>>()[0];
        

        let mut rangemap: RangeMap<u128, u128> = RangeMap::new();

        let range_mappings: Vec<Vec<u128>> = str_rangemap[1].lines()
                                                .filter(|x| !x.is_empty())
                                                .map(|x| {
                                                    x.trim().split_whitespace().map(|num| {
                                                        num.parse::<u128>().unwrap()
                                                    }).collect()
                                                })
                                                .collect();
        // println!("{:?}", range_mappings);
        for range_mapping in range_mappings {
            rangemap.insert(range_mapping[1]..range_mapping[1]+range_mapping[2], range_mapping[0]);
        }
 
        MyRangeMap {
            key: key.to_string(),
            value: value.to_string(),
            rangemap
        }
    }


    impl<'a> From<&'a str> for MyRangeMap {
        fn from(value: &'a str) -> Self {
            str_to_rangemap(value)
        }
    }
}
