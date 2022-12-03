use crate::utils::read_file;

// PUBLIC //////////////////////////////////////////////////////////////////////////////////////////
pub fn resolve_1(file_name: &str) -> i32
{
    let input = read_file(file_name);
    return find_and_accumulate_misplaced_items(&input);
}

pub fn resolve_2(file_name: &str) -> i32
{
    let input = read_file(file_name);
    return find_and_accumulate_group_badges(&input)
}





// PRIVATE /////////////////////////////////////////////////////////////////////////////////////////
fn find_and_accumulate_misplaced_items(input: &String) -> i32
{
    let mut accumulated_value = 0;
    for line in input.lines()
    {
        let misplaced_items = find_misplaced_items(&(line.to_string()));
        accumulated_value += extract_priorities(misplaced_items);
    }
    return accumulated_value;
}

fn find_misplaced_items(input: &String) -> u64
{
    let mut items_in_comp_1: u64 = 0;
    for i in 0..(input.len() / 2)
    {
        let c = input.chars().nth(i).unwrap_or_default();
        items_in_comp_1 |=  1 << get_value(c);
    }

    let mut items_in_comp_2: u64 = 0;
    for i in (input.len() / 2 )..input.len()
    {
        let c = input.chars().nth(i).unwrap_or_default();
        items_in_comp_2 |=  1 << get_value(c);
    }

    return items_in_comp_1 & items_in_comp_2;
}

fn get_value(c: char) -> u32
{
    if c.is_ascii_alphabetic()
    {
        if c.is_ascii_lowercase()
        {
            let v = c as u32;
            return v - 97;
        }
        else
        {
            return c as u32 - 39;
        }
    }
    return 0
}

fn extract_priorities(mask: u64) -> i32
{
    let mut accumulated = 0;
    for i in 0..=51
    {
        if (mask >> i) & 1 != 0
        {
            accumulated += i + 1;
        }
    }
    return accumulated;
}

fn find_and_accumulate_group_badges(input: &String) -> i32
{
    let lines: Vec<&str> = input.lines().collect();

    let mut accumulated = 0;
    for group_idx in (0..lines.len()).step_by(3) // For each group of 3 elves
    {
        let mut group_items_mask: u64 = !0;
        for elf_idx in 0..3 // For each elf in the group
        {
            let mut elf_items_mask: u64 = 0;
            for item in lines[group_idx + elf_idx].chars() // for each item in the elf's rucksack
            {
                elf_items_mask |= 1 << get_value(item);
            }

            group_items_mask &= elf_items_mask;
        }
        accumulated += extract_priorities(group_items_mask);
    }
    return accumulated;
}





// TESTS ///////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests
{
    use crate::utils::read_file;
    use super::*;

    static FILE_NAME: &str = "day3_test.txt";

    // FIRST PART //////////////////////////////////////////////////////////////
    #[test]
    fn test_full_input()
    {
        let input = read_file(FILE_NAME);
        assert_eq!(find_and_accumulate_misplaced_items(&input), 157);
    }

    #[test]
    fn test_one_line()
    {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
        assert_eq!(find_and_accumulate_misplaced_items(&input), 16);
    }

    #[test]
    fn test_find_misplaced_in_single_line()
    {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
        assert_eq!(find_misplaced_items(&input), 1 << 15);
    }

    // SECOND PART /////////////////////////////////////////////////////////////
    #[test]
    fn test_resolve_2_full_input()
    {
        let input = read_file(FILE_NAME);
        assert_eq!(find_and_accumulate_group_badges(&input), 70);
    }

    #[test]
    fn test_get_badge_of_single_group()
    {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg".to_string();
        assert_eq!(find_and_accumulate_group_badges(&input), 18);
    }
}