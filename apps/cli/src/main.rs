use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "pakdo", version = "1.0", about = "Convertisseur de fichiers optimisé et 100 % local")]
struct Args {
    /// Fichier d'entrée
    input: PathBuf,

    /// Fichier de sortie
    output: PathBuf,

    /// Librairie à utiliser pour la convertion
    #[arg(short, long)]
    lib: Option<String>,
}

fn main() {
    let args = Args::parse();

    println!("Fichier 1 : {:?}", args.input);
    println!("Fichier 2 : {:?}", args.output);
    println!("Lib : {:?}", args.lib);
}
