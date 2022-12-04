use crate::utils::read_file;

pub fn resolve_1(file_name: &str) -> i32
{
    let input = read_file(file_name);
    return find_fully_overlapped_ranges(&input).len() as i32;
}

pub fn resolve_2(file_name: &str) -> i32
{
    let input = read_file(file_name);
    return find_any_overlapped_ranges(&input).len() as i32;
}

fn find_any_overlapped_ranges(input: &String) -> Vec<(u128, u128)>
{
    let mut overlapped_ranges: Vec<(u128, u128)> = Vec::new();
    for range_pair in parse_input_ranges(input)
    {
        if range_pair.0 & range_pair.1 != 0
        {
            overlapped_ranges.push(range_pair);
        }
    }
    return overlapped_ranges;
}

fn find_fully_overlapped_ranges(input: &String) -> Vec<(u128, u128)>
{
    let mut fully_overlapped_ranges: Vec<(u128, u128)> = Vec::new();
    for range_pair in parse_input_ranges(input)
    {
        let overlapped = range_pair.0 & range_pair.1;
        if overlapped == range_pair.0 || overlapped == range_pair.1
        {
            fully_overlapped_ranges.push(range_pair);
        }
    }
    return fully_overlapped_ranges;
}

fn parse_input_ranges(input: &String) -> Vec<(u128, u128)>
{
    let mut input_ranges: Vec<(u128, u128)> = Vec::new();
    for line in input.lines()
    {
        if let Some((elf_1, elf_2)) = line.split_once(",")
        {
            let range_1 = parse_range(elf_1);
            let range_2 = parse_range(elf_2);

            input_ranges.push((range_1, range_2));
        }
    }
    return input_ranges;
}

fn parse_range(input: &str) -> u128
{
    let mut range_mask: u128 = 0;
    if let Some((min_str, max_str)) = input.split_once("-")
    {
        let min = min_str.parse::<u128>().unwrap_or(0);
        let max = max_str.parse::<u128>().unwrap_or(0);

        assert!(min > 0);
        assert!(max >= min);

        for i in (min-1)..max
        {
            range_mask |= 1 << i;
        }
    }
    return range_mask;
}

#[cfg(test)]
mod tests
{
    use crate::utils::read_file;
    use super::*;

    static FILE_NAME: &str = "day4_test.txt";

    #[test]
    fn test_parse_range()
    {
        let range = "1-4";
        assert_eq!(parse_range(range), 15); // 1111
    }

    #[test]
    fn test_parse_single_line()
    {
        let input = "2-4,6-8".to_string();
        let range_pair = parse_input_ranges(&input);

        assert_eq!(range_pair.len(), 1);
        assert_eq!(range_pair[0].0, 14);   // 0000 1110
        assert_eq!(range_pair[0].1, 224);  // 1110 0000
    }

    #[test]
    fn test_full_overlapping_single_line()
    {
        let input = "6-6,4-6".to_string();
        let overlapping_ranges = find_fully_overlapped_ranges(&input);

        assert_eq!(overlapping_ranges.len(), 1);
    }

    #[test]
    fn test_parse_full_input()
    {
        let input = read_file(FILE_NAME);
        let range_pairs = parse_input_ranges(&input);

        assert_eq!(range_pairs.len(), 6);
    }

    #[test]
    fn test_full_overlapping()
    {
        let input = read_file(FILE_NAME);
        let overlapping_ranges = find_fully_overlapped_ranges(&input);

        assert_eq!(overlapping_ranges.len(), 2);
    }

    #[test]
    fn test_some_overlapping_single_line()
    {
        let input = "5-7,7-9".to_string();
        let overlapping_ranges = find_any_overlapped_ranges(&input);

        assert_eq!(overlapping_ranges.len(), 1);
    }

    #[test]
    fn test_any_overlapping()
    {
        let input = read_file(FILE_NAME);
        let overlapping_ranges = find_any_overlapped_ranges(&input);

        assert_eq!(overlapping_ranges.len(), 4);
    }
}