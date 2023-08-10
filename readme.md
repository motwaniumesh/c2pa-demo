# C2PA Rust SDK Read/Write Demo

<br/>

## Key Functions:

`Writer::write` is a utility function that takes a file as input and outputs a copy of the file with an embedded C2PA manifest.
 
The function can also take in a list of files and output the final file in the list with a manifest composed of all of the prior files. In this case, all the prior files are considered 'ingredients' of the final manifest. 

`Reader::read` is a utility function that reads and prints information from an asset signed with a C2PA manifest file and, optionally, copies the manifest to the system clipboard or writes the manifest to a designated text file.

<br/>

## Steps to Run:


<br/>

### __Full Application Demo__
   **Demo Instructions**

<br/>
### __Binaries__
From the `rust_read_write` directory, run the quoted code in your terminal replacing the square braces and the text within with the relevant values.

1. "`cargo --bin writer -- [flags]`" 
    - "`-p '[comma-separated-paths-parent-file-last]'`" __REQUIRED__

    - "`-o [path-to-directory-to-write-output-file]`"

    - "`-n [name-of-output-file]`"

    Example:
    <br/>
        `cargo run --bin writer -- -p 'src/data/outfile.png, src/data/outfile2.png' -n output_test.png`

<br/>
<br/>

2. "`cargo --bin reader -- [flags]`"

    - "`-p [path-to-file]`" __REQUIRED__

    - "`-c`" flag to copy the manifest to your clipboard

    - "`-n`" flag to prevent outputting the manifest to the terminal

    - "`-o [output-name]`" to write the manifest to a file name/path "output-name"

    Example:
    <br/>
        `cargo run --bin reader -- -p output_test.png -cn`

<br/>
<br/>
________________________________

### TODOs:

1. ~~FIX:~~ This code seems to overwrite the manifest every time you run `cargo --bin write`. Either that or `cargo --bin read` is only returning the latest addition to the manifest.
   > This was a misunderstanding of how manifests work. Each file will only have 1 **active** manifest. However, if we use C2PA to create a new asset file with a manifest and that asset file is composed of multiple signed assets (ingredients), then the new asset will have a single manifest containing the provenance information of all of the assets (ingredients).
   >
   > As of C2PA crate version 0.20.0, write.rs and read.rs successfully signed and read pdf, mp4, jpg, and png files.
2. FEATURE: The main file should be able to create a new asset with a combined manifest, when called on two (or more) signed assets.
3. ~~FEATURE~~: The scripts should use a command line argument parser to read input.
   > Done
4. ~~Bug~~: `Reader` should not break if a manifest doesn't have assertions labelled "action".
   > Added a check so that if there aren't assertions labelled action, the code doesn't break
5. FEATURE: Build this application into a standalone read-write rust package to be deployed to AWS
