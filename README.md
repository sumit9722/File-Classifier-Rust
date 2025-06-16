# File-Classifier-Rust

Step for Running the code

- Install Rust
- Clone the github repo

Then Run Command

- `cargo run -- organize folder-path` for oragnizing
- `cargo run -- dry-run folder-path` for dry run

### Thought Process / Approch

- First, I took input from the command line by following the Rust programming book example to retrieve the command-line arguments.
- Next, I performed checks to ensure the arguments were in the expected format.
- Then, I used `read_dir()` to read the given path. For each `DirEntry`, I created a vector of structs. Each struct has a `new()` function, similar to a constructor in other OOP languages, which takes a `PathBuf` as input and extracts the file name, extension, and assigns a `Filetype` to the file.
- Based on the `Filetype`, I grouped files into separate vectors for each type.
- I then created an array of tuples containing the file type as a string and a reference to the respective vector.
- Finally:
  - If the flag is `dry-run`, the tuples are printed in order.
  - If the flag is `organize`, the folder for each existing type is created and the files are moved to those folders.
