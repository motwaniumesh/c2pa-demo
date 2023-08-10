# C2PA Rust SDK Read/Write Demo

Key Functions:
Writer::write is a utility function that takes a file as input and outputs a copy of the file with an embedded C2PA manifest.



The function can also take in a list of files and output the final file in the list with a manifest composed of all of the prior files. In this case, all the prior files are considered 'ingredients' of the final manifest.

Reader::read is a utility function that reads and prints information from an asset signed with a C2PA manifest file and, optionally, copies the manifest to the system clipboard or writes the manifest to a designated text file.


# Steps to Run:
From the rust_read_write directory, run the quoted code in your terminal replacing the square braces and the text within with the relevant values.

## Write Manifest

1. "cargo --bin writer -- [flags]"
   
      "-p '[comma-separated-paths-parent-file-last]'" REQUIRED
      
      
      "-o [path-to-directory-to-write-output-file]"
      
      
      "-n [name-of-output-file]"



Example: 

cargo run --bin writer -- -p 'src/data/outfile.png, src/data/outfile2.png' -n output_test.png

## Read Manifest

2. "cargo --bin reader -- [flags]"

"-p [path-to-file]" REQUIRED


      "-c" flag to copy the manifest to your clipboard
      
      
      "-n" flag to prevent outputting the manifest to the terminal
      
      
      "-o [output-name]" to write the manifest to a file name/path "output-name"


Example: 


cargo run --bin reader -- -p output_test.png -cn
