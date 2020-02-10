use std::{thread, time};
use log::{info, warn};
use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;
use indicatif::ProgressBar;

/// Search for a pattern in a file and display the lines that contain it.
#[allow(dead_code)]
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

// Deriving `Debug` allows it to be printed ({:?}). For user-friendly
// printing, derive `Display`.
#[derive(Debug)]
struct CustomError(String);

/// This is just an example that doesn't really do any work behind the scenes.
fn draw_progress_bar() {
    let bar = ProgressBar::new(1000);
    let delay = time::Duration::from_millis(1);
    for _ in 0..1000 {
        bar.inc(1);
        thread::sleep(delay);
    }
    bar.finish();
}

// The writer here makes it testable -- a placeholder for any type that
// implements the Write trait.
#[allow(dead_code)]
#[allow(unused)]
fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[allow(dead_code)]
fn create_string() -> &'static str {
    "hello world"
}

/// Type `cargo test` to run the tests.
#[test]
fn check_create_string() {
    assert_eq!(create_string(), "hello world");
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n")
}

/// To run it with the logger output, use this:
/// `env RUST_LOG=grrs=info cargo run -- Ok src/main.rs`
#[allow(unused_variables)]
fn main() -> Result<(), ExitFailure> {
    env_logger::init();
    info!("starting up");
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli::from_args();
    // println!("pattern = {}, path = {}", pattern, path);

    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", args.path.display()))?;

    draw_progress_bar();

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    // This just preinted the contents of the file.
    // println!("file content: {}", content);

    warn!("this could be a warning, if needed");
    info!("done");
    Ok(())
}
