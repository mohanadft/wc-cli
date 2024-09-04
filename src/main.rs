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
            print!(
                "{:value_width$} ",
                value,
                value_width = value.to_string().len()
            );
        }
    }

    println!("total");
}

#[derive(Parser, Debug)]
#[command(version, about, author)]
#[command(
    help_template = "{author-with-newline}{about-section}Version: {version} \n{usage-heading} {usage} \n{all-args}"
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

    /// files to be read
    #[arg()]
    file_name: Vec<String>,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
    type Item = (fn(usize) -> Flag, bool, fn(&'a str) -> usize);

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

fn main() {
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

    for file in &args.file_name {
        let mut new_file = File::new(file);
        let content: String = get_content(new_file.name);

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
        for flag in &file.flags {
            match flag {
                Flag::Bytes(m) => print!(
                    "{:>total_bytes$} ",
                    m,
                    total_bytes = total.bytes.to_string().len()
                ),
                Flag::Chars(c) => print!(
                    "{:>total_chars$} ",
                    c,
                    total_chars = total.chars.to_string().len()
                ),
                Flag::Lines(l) => print!(
                    "{:>total_lines$} ",
                    l,
                    total_lines = total.lines.to_string().len()
                ),
                Flag::Words(w) => print!(
                    "{:>total_words$} ",
                    w,
                    total_words = total.words.to_string().len()
                ),
                Flag::MaxLineLength(mx) => {
                    print!(
                        "{:>total_max_line$} ",
                        mx,
                        total_max_line = total.max_line_length.to_string().len()
                    )
                }
            }
        }

        print!("{}", file.name);

        println!();
    }

    if files.len() > 1 {
        print_total(&total, &args);
    }
}
