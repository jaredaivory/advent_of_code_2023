use std::fmt::Error;

pub fn start() -> Result<(), Error> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use adventofcode_2023day_05::solution;

    const SEEDS: [u32; 4]  = [79, 14,55,13];
    const MAP_SEED_SOIL: &str =  
    "seed-to-soil map:
    50 98 2
    52 50 48";

    const MAP_SOIL_FERTILIZER: &str =
    "soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15";

    const MAP_FERTILIZER_WATER: &str =
    "fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4";

    const MAP_WATER_LIGHT:  &str =
    "water-to-light map:
    88 18 7
    18 25 70";

    const MAP_LIGHT_TEMP: &str =
    "light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13";

    const MAP_TEMP_HUMIDITY: &str =
    "temperature-to-humidity map:
    0 69 1
    1 0 69";
    
    const MAP_HUMIDITY_LOCATION:  &str = 
    "humidity-to-location map:
    60 56 37
    56 93 4";

    #[test_case(MAP_SEED_SOIL)]
    #[test_case(MAP_SOIL_FERTILIZER)]
    #[test_case(MAP_FERTILIZER_WATER)]
    #[test_case(MAP_WATER_LIGHT)]
    #[test_case(MAP_LIGHT_TEMP)]
    #[test_case(MAP_TEMP_HUMIDITY)]
    #[test_case(MAP_HUMIDITY_LOCATION)]
    fn test_create_range_map(map_string: &str){
        println!("{:?}",solution::MyRangeMap::from(map_string))
    }

    #[test_case(79 => 81)]
    #[test_case(14 => 14)]
    #[test_case(55 => 57)]
    #[test_case(13 => 13)]
    fn test_seed_mapping(seed: u32) -> u32 {
        let mapping: solution::MyRangeMap = solution::MyRangeMap::from(MAP_SEED_SOIL);
        println!("{:?}", mapping);
        let res = match mapping.rangemap.get_key_value(&seed) {
            Some((range, value)) => seed - range.start + value,
            None => seed
        };

        res
    }
}