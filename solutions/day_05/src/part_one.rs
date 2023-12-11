use std::fmt::Error;

pub fn start() -> Result<(), Error> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    const SEEDS: &str  = "79 14 55 13";
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
    fn create_map(map_string: &str){
        
    }
}