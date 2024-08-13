use std::fs;

use clap::Parser;

pub fn get_content(file_name: &str) -> String {
    let a = match fs::read_to_string(&file_name) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e:?}");
            return "".to_string();
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
    #[arg(short = 'l', long = "lines")]
    lines: bool,

    /// print the words counts
    #[arg(short = 'w', long = "words")]
    words: bool,

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
            _ => None,
        };
        self.index += 1;
        result
    }
}

fn main() {
    let mut args = Args::parse();

    if !args.bytes && !args.chars && !args.lines && !args.words {
        args.bytes = true;
        args.lines = true;
        args.words = true;
    }

    let content = get_content(&args.file_name);
    let mut res = String::from("");

    let args_iter = ArgsIter::new(&args);

    for (value, fun) in args_iter {
        if value {
            res.push_str(&fun(&content).to_string());

            res.push_str(" ");
        }
    }

    println!("{}{}", res, args.file_name);
}
