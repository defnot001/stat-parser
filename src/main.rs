mod parsing;

use parsing::{get_stat_file_contents, parse_stats};
use std::time::Instant;
use std::{fs, path::Path};

fn main() {
    let now = Instant::now();

    let stat_dir_path = Path::new("./target-files/stats");
    let whitelist_path = Path::new("./target-files/whitelist.json");

    if let Some(file_paths) = get_json_paths(stat_dir_path) {
        let contents = get_stat_file_contents(file_paths);

        for stat_str in contents {
            if let Err(err) = parse_stats(&stat_str) {
                println!("Failed to parse stats: {}", err)
            };
        }
    }

    if let Ok(profiles) = parsing::get_whitelist(whitelist_path) {
        println!("{:?}", profiles)
    }

    let elapsed = now.elapsed();
    println!("Running time for the program: {:?}", elapsed);
}

fn get_json_paths(dir_path: &Path) -> Option<Vec<std::path::PathBuf>> {
    let dir = fs::read_dir(&dir_path).expect("Failed to read the stats directory");
    let mut path_vector = Vec::new();

    for entry in dir {
        match entry {
            Ok(e) => {
                if e.path().extension()?.to_str()? == "json" {
                    path_vector.push(e.path());
                }
            }
            Err(err) => {
                println!("Cannot get path: {err}");
                continue;
            }
        };
    }

    if path_vector.len() == 0 {
        panic!("There are no files with a \".json\" extension in this directory!");
    }

    Some(path_vector)
}
