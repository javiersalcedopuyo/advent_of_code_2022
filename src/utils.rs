use std::fs;
// use std::path::Path;

fn read_file(file_name: &str) -> String
{
    let path = "./inputs/".to_owned() + file_name;
    let current_path = std::env::current_dir().unwrap_or_default();
    return fs::read_to_string(path)
                .unwrap_or_default()
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