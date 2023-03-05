use std::fs::{self, ReadDir};

fn main() {
    let dir = fs::read_dir("./stats").expect("Failed to read the stats directory");

    let paths = get_paths(dir);

    println!("Paths: {:#?}", paths);
}

fn get_paths(directory: ReadDir) -> Vec<String> {
    let mut file_paths = Vec::new();

    for entry in directory {
        let path = match entry {
            Ok(path) => String::from(path.path().to_str().unwrap()),
            Err(err) => {
                println!("Cannot get path: {}", err);
                continue;
            }
        };

        if path.ends_with(".json") {
            file_paths.push(path);
        }
    }

    file_paths
}
