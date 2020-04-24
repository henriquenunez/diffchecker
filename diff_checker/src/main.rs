pub mod lcs;

use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {

    let args: Vec<String> = env::args().collect();

    let filename_a = args.get(1)
                        .expect("Filename missing!");
    let filename_b = args.get(2)
                        .expect("Filename missing!");

    println!("OwO1: {}\nOwO2: {}", filename_a, filename_b);
/*
    let string_one = vec!["aa", "bb", "cc", "cc", "cc"];//String::new();
    let string_two = vec!["aa",       "cc", "cc", "cc"];//String::new();
*/

    let file_a = File::open(Path::new(filename_a)).unwrap();
    let file_b = File::open(Path::new(filename_b)).unwrap();
    let reader_a = BufReader::new(&file_a);
    let reader_b = BufReader::new(&file_b);
    let lines_a: Vec<&str> = reader_a
                                    .lines()
                                    .collect::<Result<_, _>>()
                                    .unwrap()
                                    .map(|s| s as &str)
                                    .collect();

    let lines_b: Vec<&str> = reader_b
                                    .lines()
                                    .collect::<Result<_, _>>()
                                    .unwrap();

    let owo = lcs::create_lcs_of_strings(lines_a, lines_b);
    for grr in owo.iter() {
        println!("Part: {}", grr);
    }

}

