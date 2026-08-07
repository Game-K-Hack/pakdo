use clap::Parser;
use pakdo_core::convert;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "pakdo", version = "1.0", about = "Optimized, 100% local file converter")]
struct Args {
    /// Input file
    input: PathBuf,

    /// Output file
    output: Option<PathBuf>,

    /// in ... format
    #[arg(short, long)]
    format: Option<String>,

    /// Library to use for the conversion
    #[arg(short, long)]
    lib: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    convert(
        &args.input,                  // Passé par référence (&PathBuf -> &Path)
        args.output.as_deref(),      // Option<PathBuf> -> Option<&Path>
        args.format.as_deref(), // Option<String>  -> Option<&str>
        args.lib.as_deref()              // Option<String>  -> Option<&str>
    )?;

    Ok(())
}
