#![doc = uranus::readme_text!()]

use textwrap::{Options};
use clap::{
    Parser,
    ValueEnum,
};

/*
    reptar <text>
    reptar --width=64 <text>
    reptar 
*/

#[derive(Debug, Clone, ValueEnum)]
enum Separator {
    Ascii,
    Unicode,
}

#[derive(Debug, Clone, Copy)]
enum Width {
    Term,
    TermPercent(f64),
    Int(usize),
}

impl std::fmt::Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Width::Term => write!(f, "term"),
            Width::TermPercent(percent) => write!(f, "{percent}%"),
            Width::Int(int) => write!(f, "{int}"),
        }
    }
}

impl Width {
    fn internal_get_term_width() -> usize {
        let (width, _) = crossterm::terminal::size().expect("Failed to get terminal size.");
        width as usize
    }
    
    pub fn get(self) -> usize {
        match self {
            Width::Term => Self::internal_get_term_width(),
            Width::TermPercent(percent) => {
                let term_width = Self::internal_get_term_width() as f64;
                let percent_width = term_width * percent;
                let term_width = percent_width.floor() as usize;
                term_width
            }
            Width::Int(int) => int as usize,
        }
    }
}

impl std::str::FromStr for Width {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0"
            | "term"
            | "Term"
            | "TERM" => {
                Ok(Self::Term)
            }
            "half"
            | "Half"
            | "HALF" => {
                Ok(Self::TermPercent(0.5))
            }
            percent if percent.ends_with('%')
                 && let Ok(percent) = percent[..percent.len() - 1].parse::<f64>()
            => {
                let mul = percent / 100.0;
                Ok(Self::TermPercent(mul))
            }
            int if let Ok(int @ 1..) = int.parse::<usize>() => {
                Ok(Self::Int(int))
            }
            _ => Err("Invalid argument. Expected `term` or integer width."),
        }
    }
}

#[derive(Debug, Clone, Parser)]
struct Reptar {
    /// Break lines in the middle of words.
    #[arg(long, short)]
    break_words: bool,
    /// Whether to use regular ASCII separators, or Unicode separators for the line breaking algorithm.
    #[arg(long, short, value_enum, default_value_t = Separator::Unicode)]
    separator: Separator,
    /// The initial indent of the first paragraph.
    #[arg(long, short)]
    initial_indent: Option<String>,
    /// The width of the terminal.
    /// 
    /// Possible values are `term`, `TERM_WIDTH%`, or an integer.
    #[arg(
        long,
        short,
        default_value_t = Width::Term,
    )]
    width: Width,
    /// Whether to interpret the text_or_path argument as a file path and wrap the lines of its text.
    #[arg(long, short)]
    file: bool,
    /// The text to be wrapped or the path to the file containing the text to be wrapped (must use --file flag for path input).
    text_or_path: String,
}

#[cfg(target_os = "linux")]
macro_rules! target_os {
    () => { "linux" };
}

#[cfg(target_os = "macos")]
macro_rules! target_os {
    () => { "macos" };
}

#[cfg(target_os = "windows")]
macro_rules! target_os {
    () => { "windows" };
}

#[cfg(target_pointer_width = "64")]
macro_rules! target_pointer_width {
    () => { "64" };
}

#[cfg(target_pointer_width = "32")]
macro_rules! target_pointer_width {
    () => { "32" };
}

#[cfg(target_pointer_width = "16")]
macro_rules! target_pointer_width {
    () => { "16" };
}

const VERSION_STRING: &'static str = concat!(
    env!("CARGO_PKG_NAME"),
    " - v",
    env!("CARGO_PKG_VERSION"),
    " (",
    target_os!(),
    "-",
    target_pointer_width!(), "-bit",
    ")"
);

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 2 {
        match args[1].as_str() {
            "--version" | "-v" => {
                if let Ok(current_exe_path) = std::env::current_exe() {
                    println!("{VERSION_STRING} <\"{}\">", current_exe_path.display());
                } else {
                    println!("{VERSION_STRING}");
                }
                return;
            }
            _ => (),
        }
    }
    let args = Reptar::parse_from(args);
    let width = args.width.get();
    let mut opts = Options::new(width);
    let separator = match args.separator {
        Separator::Ascii => textwrap::WordSeparator::AsciiSpace,
        Separator::Unicode => textwrap::WordSeparator::UnicodeBreakProperties,
    };
    opts = opts.break_words(args.break_words);
    if let Some(indent) = args.initial_indent.as_deref() {
        opts = opts.initial_indent(indent);
    }
    opts = opts.word_separator(separator);

    let text = if args.file {
        std::fs::read_to_string(args.text_or_path).expect("Failed to read file.\nSorry there's not a better error message.")
    } else {
        args.text_or_path
    };
    
    let wrapped = textwrap::fill(text.as_str(), opts);
    println!("{wrapped}");
}
