

#[cfg(test)]
mod test {
    use test_case::test_case;

    static ENGINE_SCHEMATIC: &str = 
    "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    #[test_case(ENGINE_SCHEMATIC => 4361)]
    fn sum_part_numbers(engine_schematic: &str) -> u16 {
        todo!()
    }


    
}