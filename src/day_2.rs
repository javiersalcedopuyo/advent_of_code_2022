use crate::utils::read_file;

pub fn resolve_1(file_name: &str) -> i32
{
    let input = read_file(file_name);
    let mut points = 0;
    for play in parse_input(input)
    {
        points += check_play(&play) + play.1.get_value();
    }
    return points;
}

pub fn resolve_2(file_name: &str) -> i32
{
    let mut points = 0;
    for line in read_file(file_name).split("\n")
    {
        if let Some(opponent) = Shape::parse(&line.chars().nth(0).unwrap_or_default())
        {
            let player = match line.chars().nth(2).unwrap_or_default()
            {
                'X' => opponent.get_loser(),
                'Y' => opponent,
                'Z' => opponent.get_winner(),
                _ => continue
            };
            points += check_play(&(opponent, player)) + player.get_value();
        }
    }
    return points;
}

fn parse_input(input: String) -> Vec<(Shape, Shape)>
{
    let mut plays = Vec::new();

    for line in input.split("\n")
    {
        if let Some(p) = parse_line(line)
        {
            plays.push(p)
        }
    }

    return plays;
}

fn parse_line(line: &str) -> Option<(Shape, Shape)>
{
    let opponent = line.chars().nth(0);
    let player = line.chars().nth(2);

    let op = Shape::parse(&opponent.unwrap_or_default());
    let pp = Shape::parse(&player.unwrap_or_default());

    if op == None || pp == None
    {
        return None
    }
    return Some((op.unwrap(), pp.unwrap()));
}

fn check_play(play: &(Shape, Shape)) -> i32
{
    if play.0 == play.1
    {
        return 3;
    }

    match play
    {
        (Shape::Rock, Shape::Paper) |
        (Shape::Paper, Shape::Scissors) |
        (Shape::Scissors, Shape::Rock) => 6,
        _ => 0
    }
}

#[derive(Debug, PartialEq ,Eq, Copy, Clone)]
enum Shape
{
    Rock,
    Paper,
    Scissors
}

impl Shape
{
    pub fn parse(c: &char) -> Option<Self>
    {
        match c
        {
            'A' | 'X' => Some(Shape::Rock),
            'B' | 'Y' => Some(Shape::Paper),
            'C' | 'Z' => Some(Shape::Scissors),
            _ => None
        }
    }

    pub fn get_value(&self) -> i32
    {
        match self
        {
            Self::Rock      => 1,
            Self::Paper     => 2,
            Self::Scissors  => 3
        }
    }

    pub fn get_winner(&self) -> Self
    {
        match self
        {
            Self::Rock      => Self::Paper,
            Self::Paper     => Self::Scissors,
            Self::Scissors  => Self::Rock
        }
    }

    pub fn get_loser(&self) -> Self
    {
        match self
        {
            Self::Rock      => Self::Scissors,
            Self::Paper     => Self::Rock,
            Self::Scissors  => Self::Paper
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::utils::read_file;

    static FILE_NAME: &str = "day2_test.txt";

    #[test]
    fn test_resolve_1()
    {
        assert_eq!(resolve_1(FILE_NAME), 15);
    }

    #[test]
    fn test_resolve_2()
    {
        assert_eq!(resolve_2(FILE_NAME), 12);
    }

    #[test]
    fn test_parse_player_play_count()
    {
        let input = read_file(FILE_NAME);
        let plays = parse_input(input);
        assert_eq!(plays.len(), 3);
    }


    #[test]
    fn test_parse_opponent_plays()
    {
        let input = read_file(FILE_NAME);
        let plays = parse_input(input);
        assert_eq!(plays[0].0, Shape::Rock);
        assert_eq!(plays[1].0, Shape::Paper);
        assert_eq!(plays[2].0, Shape::Scissors);
    }

    #[test]
    fn test_parse_player_plays()
    {
        let input = read_file(FILE_NAME);
        let plays = parse_input(input);
        assert_eq!(plays[0].1, Shape::Paper);
        assert_eq!(plays[1].1, Shape::Rock);
        assert_eq!(plays[2].1, Shape::Scissors);
    }
}