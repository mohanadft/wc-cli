use std::{fmt::Display, fs, io};

use clap::Parser;

pub fn get_content(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(&file_name)
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
    match content.lines().max_by(|x, y| x.len().cmp(&y.len())) {
        Some(v) => v,
        None => {
            eprintln!("Content is empty.");
            std::process::exit(1);
        }
    }
    .len()
}

pub fn print_total(total: &Total, args: &Args) {
    let totals: Vec<(bool, usize)> = vec![
        (args.bytes, total.bytes),
        (args.chars, total.chars),
        (args.lines, total.lines),
        (args.words, total.words),
        (args.max_line_length, total.max_line_length),
    ];

    for (found, value) in totals {
        if found {
            print!("{} ", value);
        }
    }

    println!("total");
}

pub fn read_from_standard_input(args: &Args) {
    let args_iter2: ArgsIter = ArgsIter::new(&args);

    let content = match std::io::read_to_string(io::stdin()) {
        Ok(v) => v,
        Err(_) => {
            std::process::exit(1);
        }
    };

    for (enm, found, fun) in args_iter2 {
        if found {
            let val = enm(fun(&content));

            match val {
                Flag::Bytes(m) => print_with_width(m, m.to_string().len()),
                Flag::Chars(c) => print_with_width(c, c.to_string().len()),
                Flag::Lines(l) => print_with_width(l, l.to_string().len()),
                Flag::Words(w) => print_with_width(w, w.to_string().len()),
                Flag::MaxLineLength(mx) => print_with_width(mx, mx.to_string().len()),
            }
        }
    }
}

pub fn print_with_width<T: Display>(val: T, width: usize) {
    print!("{:>width$} ", val, width = width)
}

#[derive(Parser)]
#[command(version, about, author)]
#[command(
    help_template = "Author: {author-with-newline}{about-section}Version: {version} \n{usage-heading} {usage} \n{all-args}"
)]
pub struct Args {
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

    /// when to print a line with total counts;
    #[arg(long, default_value = "always", default_value_t = String::from("always"), value_parser=["auto", "always", "never", "only"])]
    total: String,

    /// files to be read
    #[arg()]
    file_name: Vec<String>,
}

enum Flag {
    Bytes(usize),
    Chars(usize),
    Lines(usize),
    Words(usize),
    MaxLineLength(usize),
}

#[derive(Default)]
pub struct Total {
    bytes: usize,
    chars: usize,
    lines: usize,
    words: usize,
    max_line_length: usize,
}

struct File<'a> {
    name: &'a str,
    flags: Vec<Flag>,
}

impl<'a> File<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            flags: Vec::new(),
        }
    }

    fn print_flags(&self, total: &Total) {
        for flag in &self.flags {
            match flag {
                Flag::Bytes(m) => print_with_width(m, total.bytes.to_string().len()),
                Flag::Chars(c) => print_with_width(c, total.chars.to_string().len()),
                Flag::Lines(l) => print_with_width(l, total.lines.to_string().len()),
                Flag::Words(w) => print_with_width(w, total.words.to_string().len()),
                Flag::MaxLineLength(mx) => {
                    print_with_width(mx, total.max_line_length.to_string().len())
                }
            }
        }
        print!("{}", self.name);
    }
}
#[derive(Clone, Copy)]
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
    type Item = (fn(usize) -> Flag, bool, fn(&str) -> usize);

    fn next(&mut self) -> Option<Self::Item> {
        let result: Option<(fn(usize) -> Flag, bool, fn(&str) -> usize)> = match self.index {
            0 => Some((Flag::Bytes, self.args.bytes, bytes)),
            1 => Some((Flag::Chars, self.args.chars, chars)),
            2 => Some((Flag::Lines, self.args.lines, lines)),
            3 => Some((Flag::Words, self.args.words, words)),
            4 => Some((
                Flag::MaxLineLength,
                self.args.max_line_length,
                max_line_length,
            )),
            _ => None,
        };
        self.index += 1;
        result
    }
}

fn main() -> Result<(), io::Error> {
    let mut args: Args = Args::parse();

    if [
        args.bytes,
        args.chars,
        args.lines,
        args.words,
        args.max_line_length,
    ]
    .iter()
    .all(|v| !v)
    {
        args.bytes = true;
        args.lines = true;
        args.words = true;
    }

    let args_iter: ArgsIter = ArgsIter::new(&args);

    let mut files: Vec<File> = Vec::new();

    let mut total = Total::default();

    if args.file_name.is_empty() {
        read_from_standard_input(&args);
        return Ok(());
    }

    for file in &args.file_name {
        let mut new_file = File::new(file);
        let content: String = get_content(new_file.name)?;

        for (enm, found, fun) in args_iter {
            if found {
                let val = enm(fun(&content));

                match val {
                    Flag::Bytes(m) => total.bytes += m,
                    Flag::Chars(c) => total.chars += c,
                    Flag::Lines(l) => total.lines += l,
                    Flag::Words(w) => total.words += w,
                    Flag::MaxLineLength(mx) => total.max_line_length += mx,
                }

                new_file.flags.push(val);
            }
        }

        files.push(new_file);
    }

    for file in &files {
        file.print_flags(&total);
        println!();
    }

    if files.len() > 1 && ["auto", "always"].iter().any(|v| args.total == *v) {
        print_total(&total, &args);
    }

    Ok(())
}
