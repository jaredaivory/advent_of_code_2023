
pub mod solution {
    use std::fmt::Error;

    use rangemap::RangeMap;

    #[derive(Debug)]
    pub struct MyRangeMap {
        key: String,
        value: String,
        pub rangemap: RangeMap<u32, u32>
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
        

        let mut rangemap: RangeMap<u32, u32> = RangeMap::new();

        let range_mappings: Vec<Vec<u32>> = str_rangemap[1].lines()
                                                .filter(|x| !x.is_empty())
                                                .map(|x| {
                                                    x.trim().split_whitespace().map(|num| {
                                                        num.parse::<u32>().unwrap()
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
