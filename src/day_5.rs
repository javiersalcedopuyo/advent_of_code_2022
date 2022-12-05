use crate::utils::read_file;

pub fn resolve_1(file_name: &str) -> String
{
    let input = read_file(file_name);
    let (input_stacks, instructions) = split_input(&input);
    let mut stacks = get_stacks_from_input(&input_stacks);

    apply_instructions(&mut stacks, &instructions);

    let mut stack_tops: String = "".to_string();
    for stack in stacks
    {
        if let Some(top) = stack.last()
        {
            stack_tops.push(top.to_owned());
        }
    }
    return stack_tops;
}

pub fn resolve_2(_file_name: &str) -> i32
{
    -1
}

fn split_input(input: &String) -> (String, String)
{
    let (stacks, instructions) = input.split_once("\n\n").unwrap_or_default();
    return (stacks.to_string(), instructions.to_string());
}


fn get_stacks_from_input(input: &String) -> Vec<Vec<char>>
{
    let lines: Vec<&str> = input.lines().collect();
    assert!(!lines.is_empty());

    let stack_count = (lines[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; stack_count];

    for i in (0..lines.len()-1).rev()
    {
        let line = lines[i];

        for j in (1..line.len()).step_by(4)
        {
            if let Some(c) = line.chars().nth(j)
            {
                if !c.is_whitespace()
                {
                    stacks[(j-1)/4].push(c);
                }
            }
        }
    }
    return stacks;
}

fn apply_instructions(stacks: &mut Vec<Vec<char>>, instructions: &String)
{
    for instruction in instructions.lines()
    {
        let words: Vec<&str> = instruction.split_whitespace().collect();
        assert_eq!(words.len(), 6);
        let quantity    = words[1].parse::<usize>().unwrap_or_default();
        let origin      = words[3].parse::<usize>();
        let destination = words[5].parse::<usize>();

        if origin.is_err() || destination.is_err() { continue; }

        let origin = origin.unwrap();
        assert!(origin > 0);
        let destination = destination.unwrap();
        assert!(destination > 0);

        if origin == destination { continue; }

        for _ in 0..quantity
        {
            if let Some(element) = stacks[origin-1].pop()
            {
                stacks[destination-1].push( element );
            }
        }
    }
}

#[cfg(test)]
mod tests
{
    use crate::utils::read_file;
    use super::*;

    static FILE_NAME: &str = "day5_test.txt";

    #[test]
    fn test_resolve_1()
    {
        assert_eq!(resolve_1(FILE_NAME), "CMZ")
    }

    #[test]
    fn test_split_input()
    {
        let input = read_file(FILE_NAME);
        let (stacks, instructions) = split_input(&input);

        assert!(!stacks.is_empty());
        assert!(!instructions.is_empty());
    }

    #[test]
    fn test_get_stack_count()
    {
        let input = read_file(FILE_NAME);
        let (stacks_input, _) = split_input(&input);
        let stacks = get_stacks_from_input(&stacks_input);

        assert_eq!(stacks.len(), 3);
    }

    #[test]
    fn test_get_initial_stacks()
    {
        let input = read_file(FILE_NAME);
        let (stacks_input, _) = split_input(&input);
        let stacks = get_stacks_from_input(&stacks_input);

        assert_eq!(stacks[0], vec!['Z', 'N']);
        assert_eq!(stacks[1], vec!['M', 'C', 'D']);
        assert_eq!(stacks[2], vec!['P']);
    }

    #[test]
    fn test_apply_single_instruction()
    {
        let input = read_file(FILE_NAME);
        let (stacks_input, _) = split_input(&input);
        let instructions = "move 1 from 2 to 1".to_string();
        let mut stacks = get_stacks_from_input(&stacks_input);

        apply_instructions(&mut stacks, &instructions);

        assert_eq!(stacks[0], vec!['Z', 'N', 'D']);
        assert_eq!(stacks[1], vec!['M', 'C']);
        assert_eq!(stacks[2], vec!['P']);
    }
}