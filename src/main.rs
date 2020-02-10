use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;

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

#[derive(Debug)]
struct CustomError(String);

#[allow(unused_variables)]
fn main() -> Result<(), ExitFailure> {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli::from_args();
    // println!("pattern = {}, path = {}", pattern, path);

    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", path))?;
    println!("file content: {}", content);
    Ok(())
}
