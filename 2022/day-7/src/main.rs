use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};

enum CdOption {
    In(String),
    Out,
    Root,
}

impl From<&str> for CdOption {
    fn from(value: &str) -> Self {
        match value {
            ".." => Self::Out,
            "/" => Self::Root,
            name => Self::In(name.into()),
        }
    }
}

fn sum_path_sizes(mut path: PathBuf, sizes: &mut HashMap<PathBuf, u32>, size: u32) {
    *sizes.entry(path.clone()).or_insert(0) += size;
    if path.pop() {
        sum_path_sizes(path, sizes, size);
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    // let mut current = PathBuf::from("/");
    // let mut hash: HashMap<PathBuf, u32> = HashMap::new();

    // for line in input.lines() {
    //     let mut tokens = line.trim().split_whitespace();
    //     match (tokens.next(), tokens.next(), tokens.next()) {
    //         (Some("$"), Some("cd"), Some(option)) => match option.into() {
    //             CdOption::In(dir) => current.push(dir),
    //             CdOption::Out => _ = current.pop(),
    //             CdOption::Root => current = PathBuf::from("/"),
    //         },
    //         (Some(size), Some(_file_name), None) => {
    //             sum_path_sizes(current.clone(), &mut hash, size.parse()?)
    //         }
    //         (_, _, _) => (),
    //     }
    // }

    let (_final_dir, hash) = input.lines().fold(
        (PathBuf::from("\\"), HashMap::<PathBuf, u32>::new()),
        |(mut current, mut hash), next| {
            let mut tokens = next.trim().split_whitespace();
            match (tokens.next(), tokens.next(), tokens.next()) {
                (Some("$"), Some("cd"), Some(option)) => match option.into() {
                    CdOption::In(dir) => current.push(dir),
                    CdOption::Out => _ = current.pop(),
                    CdOption::Root => current = PathBuf::from("/"),
                },
                (Some("$"), Some("ls"), None) => (),
                (Some("dir"), Some(_dir_name), None) => (),
                (Some(size), Some(_file_name), None) => sum_path_sizes(
                    current.clone(),
                    &mut hash,
                    size.parse().expect("Couldn't parse file size!"),
                ),
                (_, _, _) => (),
            };
            (current, hash)
        },
    );

    dbg!(&hash);

    #[cfg(not(feature = "day_two"))]
    {
        let result = hash
            .into_values()
            .filter(|&f| f <= 100000)
            .fold(0u32, |acc, size| acc + size);
        println!("{result}");
    }

    #[cfg(feature = "day_two")]
    {
        let needed =
            30_000_000 - (70_000_000 - hash.get(&PathBuf::from("/")).expect("No root folder!"));
        let mut dirs: Vec<(PathBuf, u32)> = hash.into_iter().collect();
        dirs.sort_by_key(|(_, v)| *v);
        let result = dirs.into_iter().find(|(_, p)| *p >= needed).unwrap();
        println!("{} : {}", result.0.display(), result.1);
    }

    Ok(())
}
