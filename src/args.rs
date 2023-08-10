use clap::{ Parser, Subcommand };

#[derive(Parser)]
#[clap(author, version, about)]
pub struct RWArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// output the manifest given the path of an asset with -p
    Read(ReadArgs),

    /// write a manifest given a file or files, outputting the last
    Write(WriteArgs),
}

#[derive(Parser)]
pub struct ReadArgs {
    /// Path to the file to be read
    #[arg(short = 'p', long = "path")]
    pub path: String,
    /// Copy contents of the manifest to the clipboard
    #[arg(short = 'c')]
    pub clip: bool,
    /// Write contents of the manifest to a new .json file at the path
    #[arg(short = 'o', long = "out")]
    pub output: Option<Option<String>>,
    /// Don't write the manifest contents to the terminal
    #[arg(short = 'n')]
    pub quiet: bool,
}

#[derive(Parser)]
pub struct WriteArgs {
    /// Path to file or files to be signed. Comma delimited eg: "/dir1/name1.ext,path2,path3"
    #[arg(short = 'p', long = "path")]
    pub path: String,
    /// Path to directory to save signed output
    #[arg(short = 'o', long = "out")]
    pub outpath: Option<String>,
    /// Custom name for the signed output
    #[arg(short = 'n', long = "name")]
    pub name: Option<String>,
}