#[path = "../args.rs"]
mod args;

use c2pa::{ ManifestStore, assertions::Actions };
use cli_clipboard::set_contents;

use std::{ error::Error, fs::File, io::Write, path::Path };
use args::{ ReadArgs };
use clap::Parser;

//Need this allow to treat the file as a standalone binary for testing
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    let args = ReadArgs::parse();
    read(&args.path, args.output, Some(args.clip), Some(args.quiet)).unwrap();
    Ok(())
}

pub fn read(
    file: &str,
    out: Option<Option<String>>,
    c: Option<bool>,
    n: Option<bool>
) -> Result<(), Box<dyn Error>> {
    let manifest_store = ManifestStore::from_file(file)?;

    if !n.unwrap_or(false) {
        println!("{}", manifest_store);
    }

    if let Some(_value) = out {
        let _path = _value.as_deref().unwrap_or("out.json");
        let path = Path::new(_path);
        let mut outfile = File::create(path)?;
        outfile.write_all(manifest_store.to_string().as_bytes())?;
        println!("Wrote the manifest of {} to {}.", file, _path);
    }

    if c.unwrap_or(false) {
        set_contents(manifest_store.to_string()).unwrap();
        println!("Copied the manifest of {} to clipboard.", file);
    }

    if let Some(manifest) = manifest_store.get_active() {
        if let Ok(actions) = manifest.find_assertion::<Actions>(Actions::LABEL) {
            for action in actions.actions {
                println!("{}\n", action.action());
            }
        }
    }
    Ok(())
}