use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{self, prelude::*};
use std::path::PathBuf;

struct JournalConfiguration {
    path: PathBuf,
}

impl JournalConfiguration {
    fn from_path(p: PathBuf) -> Self {
        JournalConfiguration { path: p }
    }
}
impl Default for JournalConfiguration {
    fn default() -> Self {
        let default = String::from("journal.txt");
        let path = PathBuf::from(default);
        JournalConfiguration { path }
    }
}

#[derive(Eq, PartialEq)]
enum Mode {
    Information,
    Context,
}

impl Mode {
    fn shorthand(&self) -> String {
        match self {
            Mode::Information => String::from("[I]"),
            Mode::Context => String::from("[C]"),
        }
    }
    fn from_char(c: char) -> Self {
        match c {
            'I' | 'i' => Mode::Information,
            'C' | 'c' => Mode::Context,
            _ => Mode::default(),
        }
    }
    fn starred(&self) -> String {
        let mut def = self.to_string();
        if self == &Mode::default() {
            def.push_str("*")
        }
        def
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Information
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Information => write!(f, "[I]nformation"),
            Mode::Context => write!(f, "[C]ontext"),
        }
    }
}

fn read_j(config: &JournalConfiguration) {
    let contents = std::fs::read_to_string(&config.path).unwrap();
    println!("Contents [{:?}]", &config.path);
    println!("{}", contents);
}

fn write_j(config: &JournalConfiguration) {
    // Open journal file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(config.path.clone())
        .unwrap();

    // Read line to be added
    let mut handle = io::stdin().lock();

    let mut content = String::new();
    loop {
        std::process::Command::new("clear").status().unwrap();

        // Print last entry and clear it
        println!("{}", content);
        let _ = io::stdout().flush();

        // Beginning of new entry w/ mode selection
        print!(
            "What do you want to do? {} {}: ",
            Mode::Context.starred(),
            Mode::Information.starred(),
        );
        let _ = io::stdout().flush();
        let mut mode = String::new();
        handle.read_line(&mut mode).unwrap();
        let mode = mode.as_str().chars().nth(0).unwrap();
        let mode = Mode::from_char(mode);

        // Loop if input invalid otherwise continue and ask for content
        print!("{}: ", mode.shorthand());
        let _ = io::stdout().flush();
        content.clear();
        handle.read_line(&mut content).unwrap();
        content.pop();

        // Write line to file
        let mut output = format!("{}: {}", mode.shorthand(), content);
        if mode == Mode::Context {
            let curr: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();
            output = format!("{} [{}]", output, curr.format("%Y-%m-%d %H:%M:%S"))
        }
        writeln!(file, "{}", output).unwrap();
    }
}

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Write to journal")]
    Write,
    #[command(about = "Read from journal")]
    Read,
}

fn main() {
    let cli = Cli::parse();
    let config = match cli.config {
        Some(p) => JournalConfiguration::from_path(p),
        _ => JournalConfiguration::default(),
    };

    match &cli.command {
        Some(Commands::Write) => write_j(&config),
        Some(Commands::Read) => read_j(&config),
        _ => {}
    }
}
