use crate::utils::read_file;

pub fn resolve_1(file_name: &str) -> i32
{
    let input = read_file(file_name);

    let mut calorie_list = get_elf_calorie_lists(&input);
    calorie_list.sort_by(|a,b| b.cmp(a));

    return calorie_list[0]
}

pub fn resolve_2(file_name: &str) -> i32
{
    let input = read_file(file_name);

    let mut calorie_list = get_elf_calorie_lists(&input);
    calorie_list.sort_by(|a,b| b.cmp(a));

    assert!(calorie_list.len() >= 3);

    return calorie_list[0] + calorie_list[1] + calorie_list[2]
}

fn get_elf_calorie_lists(input: &String) -> Vec<i32>
{
    let mut calorie_list: Vec<i32> = Vec::new();
    for block in input.split("\n\n")
    {
        calorie_list.push( count_elf_calories(block) );
    }
    return calorie_list;
}

fn count_elf_calories(list_str: &str) -> i32
{
    let mut total_calories = 0;
    for line in list_str.split("\n")
    {
        total_calories += line.to_string()
                              .parse::<i32>()
                              .unwrap_or(0);
    }
    return total_calories;
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::utils::read_file;

    static FILE_NAME: &str = "day1_test.txt";

    #[test]
    fn test_resolve_1()
    {
        assert_eq!(resolve_1(FILE_NAME), 24000);
    }

    #[test]
    fn test_resolve_2()
    {
        assert_eq!(resolve_2(FILE_NAME), 45000);
    }

    #[test]
    fn test_count_elves()
    {
        let input = read_file(FILE_NAME);
        let elves = get_elf_calorie_lists(&input);
        assert_eq!(elves.len(), 5)
    }

    #[test]
    fn test_get_elf_calories()
    {
        let input = read_file(FILE_NAME);
        assert_eq!(get_elf_calorie_lists(&input)[0], 6000);
    }
}