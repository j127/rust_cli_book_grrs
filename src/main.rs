use structopt::StructOpt;

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

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli::from_args();
    // println!("pattern = {}, path = {}", pattern, path);

    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into()); }
    };

    println!("file content: {}", content);
    Ok(())

}
