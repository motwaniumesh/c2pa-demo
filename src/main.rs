mod args;
mod bin;

use args::{ RWArgs, Commands };
use clap::Parser;
use bin::reader::read;
//use bin::writer::write;

fn main() {
    let args = RWArgs::parse();
    match &args.command {
        Commands::Read(read_args) => {
            read(
                &read_args.path[..],
                read_args.output.clone(),
                Some(read_args.clip),
                Some(read_args.quiet)
            ).unwrap();
        }
        Commands::Write(write_args) => {
            let paths_vec: Vec<String> = write_args.path
                .split(',')
                .map(|s| s.trim().to_owned())
                .collect();
            // TODO: check that there is at least one path
            println!("Here are your paths: {:?}!!!!!!", paths_vec)
        }
    }
}