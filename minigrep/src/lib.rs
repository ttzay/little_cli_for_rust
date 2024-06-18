
use search::{search_directory, search_file};
use clap::{ Arg, ArgAction, Command};



fn parse_args() -> clap::ArgMatches {
    Command::new("minigrep")
        .author("zhou_yuz")
        .version("version 0.1.0")
        .about("Searches for a string in files in a directory.")
        .arg(
            Arg::new("filepath")
                .help("Set the path to the file to search")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("search_string")
                .help("Set the search string")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .help("Searches for the search string in all files in the specified directory,not recursively")
                .action(ArgAction::SetTrue),
        )
        .get_matches()
}

fn match_directory_and_file_search(matches: &clap::ArgMatches) {
    match matches.get_flag("directory") {
        true => {
            let directory_path = matches.get_one::<String>("filepath").expect("directory path is required");
            let search_string = matches.get_one::<String>("search_string").expect("search string is required");
            let result = search_directory(&directory_path, &search_string).expect("An error occurred while searching the directory.");
            for (file_name, lines) in result {
                println!("{}:", file_name);
                for (line, line_numbers) in lines {
                    println!("{}: {:?}", line, line_numbers);
                }
            }
        }
        false => {
            let search_string = matches.get_one::<String>("search_string").expect("search string is required");
            let file_path = matches.get_one::<String>("filepath").expect("file path is required");
            let result = search_file(file_path, search_string).expect("An error occurred while searching the file.");
            for (line, line_numbers) in result {
                println!("{}: {:?}", line, line_numbers);
            }
        }
    }
}

pub fn run()  {
    let matches = parse_args();
    match_directory_and_file_search(&matches);
}



/// This module provides functions for searching files and directories for a given search string.
pub mod search {
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    /// Searches a file for occurrences of a search string and returns a HashMap containing the lines and their corresponding line numbers.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the file to be searched.
    /// * `search_string` - The string to search for in the file.
    ///
    /// # Returns
    ///
    /// A HashMap where the keys are the lines containing the search string and the values are vectors of line numbers where the search string was found.
    pub fn search_file(file_path: &str, search_string: &str) -> Result<HashMap<String, Vec<usize>>, std::io::Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut line_number = 0;
        let mut result = HashMap::new();
        for line in reader.lines() {
            line_number += 1;
            let line = match line {
                Ok(content) => content,
                Err(_) => {
                    // 将无效的 UTF-8 数据处理为替代字符
                    let file_bytes = fs::read(file_path)?;
                    let content = String::from_utf8_lossy(&file_bytes);
                    content.to_string()
                }
            };
            if line.contains(search_string) {
                let entry = result.entry(line.clone()).or_insert(vec![]);
                entry.push(line_number);
            }
        }
        Ok(result)
    }
    /// Searches a directory for files containing occurrences of a search string and returns a HashMap containing the file names, lines, and their corresponding line numbers.
    ///
    /// # Arguments
    ///
    /// * `directory_path` - The path to the directory to be searched.
    /// * `search_string` - The string to search for in the files.
    ///
    /// # Returns
    ///
    /// A HashMap where the keys are the file names, and the values are HashMaps where the keys are the lines containing the search string and the values are vectors of line numbers where the search string was found.
    pub fn search_directory(directory_path: &str, search_string: &str) -> Result<HashMap<String, HashMap<String, Vec<usize>>>, std::io::Error> {
        let path = Path::new(directory_path);
        let mut result = HashMap::new();
        for entry in path.read_dir()? {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let file_path = path.to_str().unwrap();
                // let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                let search_result = search_file(file_path, search_string)?;
                if !search_result.is_empty() {
                    result.insert(file_path.to_string(), search_result);
                }
            }
            // if path.is_dir() {
            //     let directory_path = path.to_str().unwrap();
            //     let search_result = search_directory(directory_path, search_string)?;
            //     if !search_result.is_empty() {
            //         result.extend(search_result);
            //     }
            // }
        }
        Ok(result)
    }
}

// use tempfile::tempdir;
// #[test]
// fn test_search_file() {
//     let temp_dir = tempdir().unwrap();
//     let file_path = temp_dir.path().join("testfile.txt");
//     let mut file = File::create(&file_path).unwrap();
//     writeln!(file, "safe, fast, productive.").unwrap();
//     writeln!(file, "Pick three.").unwrap();

//     let result = search_file(file_path.to_str().unwrap(), "fast").unwrap();
//     let expected: HashMap<String, Vec<usize>> = vec![
//         ("safe, fast, productive.".to_string(), vec![1])
//     ].into_iter().collect();
//     assert_eq!(result, expected);
// }

// #[test]
// fn test_search_directory() {
//     let temp_dir = tempdir().unwrap();
//     let file_path1 = temp_dir.path().join("testfile1.txt");
//     let mut file1 = File::create(&file_path1).unwrap();
//     writeln!(file1, "safe, fast, productive.").unwrap();
//     writeln!(file1, "Pick three.").unwrap();

//     let file_path2 = temp_dir.path().join("testfile2.txt");
//     let mut file2 = File::create(&file_path2).unwrap();
//     writeln!(file2, "Rust: safe, fast, productive.").unwrap();
//     writeln!(file2, "Pick three.").unwrap();

//     let result = search_directory(temp_dir.path(), "productive").unwrap();
//     let expected: HashMap<String, HashMap<String, Vec<usize>>> = vec![
//         (
//             file_path1.to_str().unwrap().to_string(),
//             vec![("safe, fast, productive.".to_string(), vec![1])]
//                 .into_iter()
//                 .collect(),
//         ),
//         (
//             file_path2.to_str().unwrap().to_string(),
//             vec![("Rust: safe, fast, productive.".to_string(), vec![1])]
//                 .into_iter()
//                 .collect(),
//         ),
//     ]
//     .into_iter()
//     .collect();
//     assert_eq!(result, expected);
// }
