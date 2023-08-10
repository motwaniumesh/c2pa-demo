#[path = "../args.rs"]
mod args;

use c2pa::{
    assertions::{ User, Actions, Action, c2pa_action },
    create_signer,
    Manifest,
    ManifestStore,
    SigningAlg,
    Ingredient,
};
use args::WriteArgs;
use std::{ ffi::OsStr, path::PathBuf, error::Error };
use clap::Parser;

#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    let args = WriteArgs::parse();

    // File paths to merge into the final
    let paths_vec: Vec<String> = args.path
        .split(',')
        .map(|s| s.trim().to_owned())
        .collect();

    if paths_vec.len() > 0 {
        return write(paths_vec, args.outpath, args.name);
    }
    Ok(())
}

pub fn write(
    files: Vec<String>,
    outpath: Option<String>,
    name: Option<String>
) -> Result<(), Box<dyn Error>> {
    if files.len() == 1 {
        return new_manifest(files[0].clone(), outpath, name);
    }
    return reduce_manifest(files, outpath, name);
}

// Would ideally accept and parse some data structure containing:
// Assertions:
//      Challenges:
//              1.  Assertions can be damn near anything.
//                  Maybe they should be sent in as
//                  a.  byte stream, file name pair
//                  b.  Text/json
//              2.  These should be hashed
//              3.  Need to have a jumbf URI for the claim
//
// Claims:
//      Challenges:
//              1.  Why wasn't this already in the demo generated??
//
// Signature Credentials:
//      TODO: Check the arguments for the create_signer::from_files
//
//
// Notes:
// Use "c2pa-rs/sdk/tests/integration.rs" for how to create a manifest store for multiple assets
// Might need to gather use cases or even a single use case from Umesh.
//        What are the companies we would like to implement this for doing?
// Questions:
//      Does the actor create the claim themselves, or does it happen automatically?
//      If not, why isn't claim generation in any of the demos?
fn new_manifest(
    file: String,
    outpath: Option<String>,
    name: Option<String>
) -> Result<(), Box<dyn Error>> {
    let mut manifest = Manifest::new("write".to_owned());
    manifest.add_assertion(&User::new("org.contentauth.mylabel", "Random things"))?;
    manifest.add_assertion(&User::new("datetime", "Tue, April 18th 3:30pm"))?;

    //Constructing the file paths
    let source = PathBuf::from(&file);
    let dest = outfile_helper(&source, outpath, name);

    println!(" source: {:?}, destination: {:?} ", &source, &dest);

    // Create a ps256 signer using certs and key files
    let signcert_path = "src/data/test-certs/ps256.pub";
    let pkey_path = "src/data/test-certs/ps256.pem";
    let signer = create_signer::from_files(signcert_path, pkey_path, SigningAlg::Ps256, None)?;

    // embed a manifest using the signer
    manifest.embed(&source, &dest, &*signer)?;
    Ok(())
}

fn reduce_manifest(
    files: Vec<String>,
    outpath: Option<String>,
    name: Option<String>
) -> Result<(), Box<dyn Error>> {
    //Split the source file and the ingredients iles
    let (parent_path, ingredients) = files.split_last().unwrap();

    //Creating a new manifest and associated action items
    let mut manifest = Manifest::new("write".to_owned());
    let mut actions = Actions::new();

    //Adding the ingredients and associated actions
    for _ingredient in ingredients.iter() {
        let ingredient = Ingredient::from_file(&_ingredient)?;
        let prior = ManifestStore::from_file(&_ingredient);

        //Try to add an assertion for the ingredient file's active manifest
        match prior {
            Ok(value) => {
                manifest.add_assertion(
                    &User::new(
                        &format!("Stringified manifest of {}", &_ingredient),
                        &value.get_active().unwrap().to_string()
                    )
                )?;
            }
            Err(_) => (),
        }

        //Add an assertion that we imported the file
        actions = actions.add_action(
            Action::new(c2pa_action::EDITED)
                .set_parameter("name".to_owned(), "import")?
                .set_parameter("identifier".to_owned(), ingredient.instance_id().to_owned())?
        );
        manifest.add_ingredient(ingredient);
    }
    manifest.add_assertion(&actions)?;

    manifest.set_parent(Ingredient::from_file(&parent_path)?)?;

    // Create a ps256 signer using certs and key files
    let signcert_path = "src/data/test-certs/ps256.pub";
    let pkey_path = "src/data/test-certs/ps256.pem";
    let signer = create_signer::from_files(signcert_path, pkey_path, SigningAlg::Ps256, None)?;

    // embed a manifest using the signer
    let source = PathBuf::from(&parent_path);
    let dest = outfile_helper(&source, outpath, name);

    manifest.embed(&source, &dest, &*signer)?;

    Ok(())
}

//Helps create a default output file path
fn outfile_helper(source: &PathBuf, path: Option<String>, name: Option<String>) -> PathBuf {
    let mut dest = PathBuf::from(path.as_deref().unwrap_or(""));
    if let Some(value) = name {
        dest.push(value);
    } else {
        dest.push("outfile.".to_owned() + source.extension().and_then(OsStr::to_str).unwrap());
    }

    return dest;
}