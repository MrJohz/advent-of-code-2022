pub struct Day7;

#[derive(PartialEq, Eq, Debug)]
enum Line<'a> {
    ChangeDir(&'a str),
    ChangeUpDir,
    ListDir,
    FoundDirectory(&'a str),
    FoundFile(&'a str, usize),
}

impl<'a> Line<'a> {
    fn parse(input: &'a str) -> Self {
        if input.starts_with("$ l") {
            Self::ListDir
        } else if input.starts_with("d") {
            Self::FoundDirectory(&input[4..])
        } else if input == "$ cd .." {
            Self::ChangeUpDir
        } else if input.starts_with("$ cd") {
            Self::ChangeDir(&input[5..])
        } else {
            let (nums, file) = input.split_once(' ').unwrap();
            Self::FoundFile(file, nums.parse().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_change_up_directory() {
        assert_eq!(Line::parse("$ cd .."), Line::ChangeUpDir);
    }

    #[test]
    fn test_parses_change_directory() {
        assert_eq!(Line::parse("$ cd hello"), Line::ChangeDir("hello"));
    }

    #[test]
    fn test_parses_directory_output() {
        assert_eq!(Line::parse("dir hello"), Line::FoundDirectory("hello"));
    }

    #[test]
    fn test_parses_file_output() {
        assert_eq!(
            Line::parse("14848514 b.txt"),
            Line::FoundFile("b.txt", 14848514)
        );
    }

    #[test]
    fn test_parses_list_directory() {
        assert_eq!(Line::parse("$ ls"), Line::ListDir);
    }
}

fn collect_directory_sizes(input: &str) -> Vec<usize> {
    let mut found_directories = Vec::new();
    let mut current_path = Vec::new();

    for line in input.lines().map(Line::parse) {
        match line {
            Line::ChangeDir(_) => current_path.push(0),
            Line::ChangeUpDir => found_directories.push(current_path.pop().unwrap()),
            Line::ListDir => {}
            Line::FoundDirectory(_) => {}
            Line::FoundFile(_, size) => {
                for path in &mut current_path {
                    *path += size;
                }
            }
        }
    }

    current_path.append(&mut found_directories);
    // the first directory in the list will be the root directory
    current_path
}

impl crate::runner::Day for Day7 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        Ok(format!(
            "{}",
            collect_directory_sizes(input)
                .iter()
                .filter(|&size| *size <= 100000)
                .sum::<usize>()
        ))
    }
    fn expected_value_part_1() -> Option<&'static str> {
        Some("1783610")
    }

    fn part_2(input: &str) -> anyhow::Result<String> {
        let directory_sizes = collect_directory_sizes(input);

        let total_size = 70_000_000;
        let required_size = 30_000_000;
        let current_size: usize = directory_sizes[0];
        let to_be_freed = required_size - (total_size - current_size);

        let options = collect_directory_sizes(input)
            .iter()
            .filter(|&size| *size > to_be_freed)
            .fold(total_size, |acc, &size| if acc > size { size } else { acc });

        Ok(format!("{}", options))
    }
    fn expected_value_part_2() -> Option<&'static str> {
        Some("4370655")
    }
}
