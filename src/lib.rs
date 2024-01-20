use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long = "number",
        help = "Number lines",
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,

    #[arg(short = 'b', long = "number-nonblank", help = "Number non-blank lines")]
    number_nonblank_lines: bool,
}

pub fn get_args() -> Result<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> Result<()> {
    for filename in args.files {
        println!("{filename}");
    }
    Ok(())
}
