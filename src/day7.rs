pub struct Day7;

#[derive(PartialEq, Eq, Debug)]
enum Line {
    ChangeDir,
    ChangeUpDir,
    FoundFile(usize),
}

impl Line {
    fn parse(input: &str) -> Option<Self> {
        if input.starts_with("$ l") || input.starts_with('d') {
            None
        } else if input == "$ cd .." {
            Some(Self::ChangeUpDir)
        } else if input.starts_with("$ cd") {
            Some(Self::ChangeDir)
        } else {
            let (nums, _) = input.split_once(' ').unwrap();
            Some(Self::FoundFile(nums.parse().unwrap()))
        }
    }
}

fn collect_directory_sizes(input: &str) -> Vec<usize> {
    let mut found_directories = Vec::new();
    let mut current_path = Vec::new();

    for line in input.lines().map(Line::parse) {
        match line {
            None => {}
            Some(Line::ChangeDir) => current_path.push(0),
            Some(Line::ChangeUpDir) => found_directories.push(current_path.pop().unwrap()),
            Some(Line::FoundFile(size)) => {
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
