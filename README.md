# `wc-cli` — A Rust-Powered Word Count Utility 🚀

Welcome to wc-cli, a blazing-fast reimagining of the classic wc command, written in Rust! 🦀
This project is inspired by Unix's wc command, and it's here to help you count words, lines, characters, and more—quickly and efficiently.

## 🚀 Features

- [x] Line Count (-l): Count the number of lines in a file.
- [x] Word Count (-w): Get the word count with precision.
- [x] Character Count (-c): How many characters are in your file? Find out!
- [x] Longest Line (-L): Measure the length of the longest line for optimized file reading.
- [x] Bytes Count (-m): Count the number of bytes in a file.
- [ ] Total (--total): Show total counts across multiple files with always, never, or auto modes.

## 📦 Installation

Installing wc-cli is easy! First, make sure you have Rust installed on your machine. Then, run the following commands:

```bash
  git clone https://github.com/mohanadft/wc-cli.git # Clone the Repo
  cd wc-cli                                         # Go inside the package
  cargo install --path .                            # Install the package globally
```

## ⌨️ Usage

Simple and intuitive to use! Just like the Unix wc command, but better 😉.

```bash
  wc-cli [OPTIONS] [FILE_NAME]...
  Arguments:
    [FILE_NAME]...  files to be read

  Options:
    -c, --bytes            print the byte counts
    -m, --chars            print the chars counts
    -l, --lines            print the lines counts
    -w, --words            print the words counts
    -L, --max-line-length  print the maximum display width
    --total <TOTAL>    when to print a line with total counts; [default: always] [possible values: auto, always, never]
    -h, --help             Print help
    -V, --version          Print version
```

## 🌟 Why Rust?

Rust's zero-cost abstractions and memory safety make wc-cli not only fast but also reliable and safe for handling large files without breaking a sweat.

## 🤝 Contributing

Want to add new features, fix bugs, or improve the codebase? PRs are welcome! Check out our CONTRIBUTING.md to get started.
