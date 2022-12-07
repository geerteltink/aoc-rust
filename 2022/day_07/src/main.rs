use indextree::{Arena, NodeId};

static DAY: &'static str = "07";

enum Entry {
    #[allow(dead_code)]
    File {
        name: String,
        size: usize,
    },
    Directory {
        name: String,
    },
}

fn main() {
    let input = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(input: &String) -> usize {
    let (filesystem, directories, _) = create_filesystem(input);

    // Get size for each directory
    let directory_sizes: Vec<usize> = directories
        .iter()
        .map(|d| disk_usage(&filesystem, d))
        .collect();

    // Get the dir size sum of all directories less than 100000
    let answer: usize = directory_sizes
        .iter()
        .filter(|&size| *size <= 100_000)
        .sum();

    return answer;
}

fn part_two(input: &String) -> usize {
    let (filesystem, directories, root) = create_filesystem(input);

    // Get size for each directory
    let directory_sizes: Vec<usize> = directories
        .iter()
        .map(|d| disk_usage(&filesystem, d))
        .collect();

    let free_space = 70_000_000 - disk_usage(&filesystem, &root);
    let at_least_needed = 30_000_000 - free_space;

    let answer: &usize = directory_sizes
        .iter()
        .filter(|&&size| size >= at_least_needed)
        .min()
        .unwrap();

    return *answer;
}

fn create_filesystem(input: &String) -> (Arena<Entry>, Vec<NodeId>, NodeId) {
    let mut filesystem = Arena::new();
    let mut directories: Vec<NodeId> = vec![];

    let root = filesystem.new_node(Entry::Directory {
        name: "/".to_string(),
    });

    let mut pwd = root;

    for line in input.lines() {
        let args: Vec<&str> = line.split_ascii_whitespace().collect();
        match args[..] {
            ["$", "cd", "/"] => pwd = root,
            ["$", "cd", ".."] => pwd = filesystem[pwd].parent().unwrap(),
            ["$", "cd", dir_name] => {
                pwd = pwd
                    .children(&filesystem)
                    .find(|&child| matches!(filesystem[child].get(), Entry::Directory { name } if name == dir_name))
                    .unwrap();
            }
            ["$", "ls"] => (),
            ["dir", name] => {
                let directory = filesystem.new_node(Entry::Directory {
                    name: name.to_string(),
                });
                directories.push(directory);
                pwd.append(directory, &mut filesystem);
            }
            [size, file_name] => {
                let size: usize = size.parse().unwrap();
                let file = filesystem.new_node(Entry::File {
                    name: file_name.to_string(),
                    size,
                });
                pwd.append(file, &mut filesystem);
            }
            _ => panic!("Invalid input {}", line),
        }
    }

    return (filesystem, directories, root);
}

fn disk_usage(filesystem: &Arena<Entry>, entry: &NodeId) -> usize {
    match filesystem[*entry].get() {
        Entry::File { name: _, size } => *size,
        Entry::Directory { name: _ } => entry
            .children(filesystem)
            .map(|child| match filesystem[child].get() {
                Entry::File { name: _, size } => *size,
                Entry::Directory { name: _ } => disk_usage(filesystem, &child),
            })
            .sum(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(95437, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(24933642, part_two(&input));
    }
}
