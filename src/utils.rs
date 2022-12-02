use std::fs;
// use std::path::Path;

pub fn read_file(file_name: &str) -> String
{
    let path = "./inputs/".to_owned() + file_name;
    if let Ok(contents) = fs::read_to_string(&path)
    {
        return contents;
    }
    else
    {
        println!("⛔️ ERROR: File {} not found!", path);
        return "".to_string();
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_reading_file()
    {
        let input = read_file("test.txt");
        assert_eq!(input, "Hello World!");
    }
}