#[derive(Debug)]
struct File<'a> {
    _name: &'a str,
    size: usize,
}

impl<'a> From<&'a str> for File<'a> {
    fn from(value: &'a str) -> Self {
        let (size, name) = value.split_once(' ').unwrap();
        Self {
            _name: name,
            size: size.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Directory<'a> {
    name: &'a str,
    subdirectories: Vec<Directory<'a>>,
    files: Vec<File<'a>>,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            subdirectories: Vec::new(),
            files: Vec::new(),
        }
    }

    fn root() -> Self {
        Self::new("/")
    }

    fn get_mut(&mut self, path: &[&str]) -> &mut Self {
        let Some(x) = path.first() else {
            return self;
        };
        self.subdirectories
            .iter_mut()
            .find(|it| it.name == *x)
            .unwrap()
            .get_mut(&path[1..])
    }

    fn size(&self) -> usize {
        self.subdirectories
            .iter()
            .map(|subdir| subdir.size())
            .chain(self.files.iter().map(|file| file.size))
            .sum()
    }
}

impl<'a> TryFrom<&'a str> for Directory<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (dir, name) = value.split_once(' ').ok_or(())?;
        if dir != "dir" {
            return Err(());
        }
        Ok(Self::new(name))
    }
}

enum Command<'a> {
    Cd(&'a str),
    Ls {
        directories: Vec<Directory<'a>>,
        files: Vec<File<'a>>,
    },
}

impl<'a> From<&'a str> for Command<'a> {
    fn from(value: &'a str) -> Self {
        let (cmd, rest) = value.split_at(2);
        let rest = &rest[1..];
        match cmd {
            "cd" => Self::Cd(rest),
            "ls" => {
                let mut directories = Vec::new();
                let mut files = Vec::new();
                for line in rest.lines() {
                    if let Ok(dir) = Directory::try_from(line) {
                        directories.push(dir);
                    } else {
                        files.push(File::from(line));
                    }
                }
                Self::Ls { directories, files }
            }
            _ => panic!("unknown command {cmd}"),
        }
    }
}

struct FileSystem<'a>(Directory<'a>);

impl<'a> From<&'a str> for FileSystem<'a> {
    fn from(value: &'a str) -> Self {
        let mut fs = Directory::root();
        let mut current_path = Vec::new();
        for command in value[2..].split("\n$ ").skip(1).map(Command::from) {
            match command {
                Command::Cd(name) => {
                    if name == ".." {
                        current_path.pop();
                    } else {
                        current_path.push(name);
                    }
                }
                Command::Ls { directories, files } => {
                    let mut dir = fs.get_mut(&current_path);
                    dir.subdirectories = directories;
                    dir.files = files;
                }
            }
        }
        FileSystem(fs)
    }
}

fn shitty_iter<'a>(dir: &'a Directory<'a>) -> Vec<&'a Directory<'a>> {
    dir.subdirectories
        .iter()
        .chain(
            dir.subdirectories
                .iter()
                .flat_map(|subdir| shitty_iter(subdir).into_iter()),
        )
        .collect()
}

pub fn part1(input: &str) -> usize {
    let FileSystem(root) = input.into();
    shitty_iter(&root)
        .into_iter()
        .map(|dir| dir.size())
        .filter(|size| *size < 100_000)
        .sum()
}

const TOTAL_DISK_SPACE: usize = 70_000_000;
const REQUIRED_SIZE: usize = 30_000_000;

pub fn part2(input: &str) -> usize {
    let FileSystem(root) = input.into();
    let used_space = root.size();
    let free_space = TOTAL_DISK_SPACE - used_space;
    let min_delete_space = REQUIRED_SIZE - free_space;
    shitty_iter(&root)
        .into_iter()
        .map(|dir| dir.size())
        .filter(|size| *size >= min_delete_space)
        .min()
        .unwrap()
}

// #[cfg(test)]
// mod unit_tests {
//     use super::*;

//     #[test]
//     fn sanity_check() {
//         assert_eq!(1 + 1, 2)
//     }
// }
