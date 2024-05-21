use std::error::Error;
use std::path::PathBuf;
use std::{fs, io, path::Path};

pub struct Config {
    pub dir_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 1 {
            return Err("Needs one more argument!");
        } else if args.len() > 2 {
            dbg!("{args}");
            return Err("Only one argument allowed at this time!");
        }

        let file_path = args[1].clone();
        Ok(Config {
            dir_path: file_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // we start by just printing the json files found
    let paths: Vec<PathBuf> = json_paths(config.dir_path)?;
    println!("{:#?}", paths);
    Ok(())
}

/// Iterate over a directory and find json files
/// that we use as character "sheets".
pub fn json_paths(dir: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    // Code originally written by Carnagion#5942 @ Rust Prog Lang Community Discord.
    // Explanations by harudagondi#1480.
    // `fs::read_dir` return a result of `ReadDir`, which is an iterator of `Result<DirEntry, Error>`.
    fs::read_dir(dir)?
        // a `filter_map` is a `filter` and a `map` combined, as indicated in the name. the filtering mechanism is done through returning an `Option`: `Some` if you want to keep the value, `None` to discard it.
        .filter_map(|entry|
                // `entry.map` here means it calls `Result::map`, which is essentially just turning `Result<T, E>` to `Result<U, E>` or in other words, mapping the success value to a different value (with possibly a different type)
                entry.map(|entry| {
                    // inside the `map`, carnagion gets the path of the directory, then its extension, then returns a boolean whether or not the `extension()` returned a Some with a value of "json". `then_some` just turns a boolean into a `Some` with the value given if it is true, `None` otherwise
            let path = entry.path();
            let is_json = path.extension().is_some_and(|ext| ext == "json");
            is_json.then_some(path)
        // going back, `entry.map` in this context turns a `Result<DirEntry, Error>` into `Result<Option<PathBuf>, Error>`. however, `filter_map` wants an `Option`. `transpose` just turns the result inside out so it becomes `Option<Result<PathBuf, Error>>`
        }).transpose())
        .collect()
}

#[cfg(test)]
mod tests {
    // use super::*;
}
