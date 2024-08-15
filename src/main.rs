use std::fs;

use clap::Parser;

pub fn get_content(file_name: &str) -> String {
    let a: String = match fs::read_to_string(&file_name) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        }
    };

    return a;
}

pub fn bytes(content: &str) -> usize {
    content.as_bytes().len()
}

pub fn lines(content: &str) -> usize {
    content.lines().count()
}

pub fn chars(content: &str) -> usize {
    content.chars().count()
}

pub fn words(content: &str) -> usize {
    content.split_whitespace().count()
}

pub fn max_line_length(content: &str) -> usize {
    content
        .lines()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .expect("Content is empty, no lines to compare.")
        .len()
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// print the byte counts
    #[arg(short = 'c', long = "bytes")]
    bytes: bool,

    /// print the chars counts
    #[arg(short = 'm', long = "chars")]
    chars: bool,

    /// print the lines counts
    #[arg(short, long = "lines")]
    lines: bool,

    /// print the words counts
    #[arg(short, long = "words")]
    words: bool,

    /// print the maximum display width
    #[arg(short = 'L', long = "max-line-length")]
    max_line_length: bool,

    /// file to be read
    #[arg()]
    file_name: String,
}

struct ArgsIter<'a> {
    args: &'a Args,
    index: usize,
}

impl<'a> ArgsIter<'a> {
    fn new(args: &'a Args) -> Self {
        ArgsIter { args, index: 0 }
    }
}

impl<'a> Iterator for ArgsIter<'a> {
    type Item = (bool, fn(&'a str) -> usize);

    fn next(&mut self) -> Option<Self::Item> {
        let result: Option<(bool, fn(&str) -> usize)> = match self.index {
            0 => Some((self.args.bytes, bytes)),
            1 => Some((self.args.chars, chars)),
            2 => Some((self.args.lines, lines)),
            3 => Some((self.args.words, words)),
            4 => Some((self.args.max_line_length, max_line_length)),
            _ => None,
        };
        self.index += 1;
        result
    }
}

fn main() {
    let mut args: Args = Args::parse();

    if !args.bytes && !args.chars && !args.lines && !args.words && !args.max_line_length {
        args.bytes = true;
        args.lines = true;
        args.words = true;
    }

    let content: String = get_content(&args.file_name);
    let mut res: String = String::from("");

    let args_iter: ArgsIter = ArgsIter::new(&args);

    for (value, fun) in args_iter {
        if value {
            res.push_str(&fun(&content).to_string());

            res.push_str(" ");
        }
    }

    println!("{}{}", res, args.file_name);
}
