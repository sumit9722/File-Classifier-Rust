use std::{env, fs, path::PathBuf, process::ExitCode};

#[derive(Debug, Clone)]
enum Filetype {
    Images,
    Documents,
    Video,
    Audio,
    Archives,
    Others,
}

#[derive(Debug, Clone)]
struct File {
    path: PathBuf,
    path_name: String,
    name: String,
    extension: String,
    file_type: Filetype,
}

impl File {
    fn new(path: PathBuf) -> Self {
        let path_name = path.display().to_string();
        let n: usize = extract(&path_name, '/', '\\') + 1;
        let name: String = String::from(&path_name[n..]);
        let e: usize = extract(&name, '.', '.');
        let extension: String = String::from(&name[e..]);
        let file_type = classify_file_type(&extension);

        let temp = File {
            path,
            path_name,
            name,
            extension,
            file_type,
        };
        return temp;
    }
}

fn classify_file_type(extension: &String) -> Filetype {
    match extension.to_lowercase().as_str() {
        ".jpg" | ".png" | ".gif" | ".bmp" | ".svg" => Filetype::Images,
        ".pdf" | ".docx" | ".txt" | ".md" => Filetype::Documents,
        ".mp4" | ".avi" | ".mkv" | ".mov" => Filetype::Video,
        ".mp3" | ".wav" | ".flac" => Filetype::Audio,
        ".zip" | ".rar" | ".tar" | ".gz" => Filetype::Archives,
        _ => Filetype::Others,
    }
}

fn extract(s: &String, check1: char, check2: char) -> usize {
    let bytes = s.as_bytes();
    let mut pos: usize = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == check1 as u8 || item == check2 as u8 {
            pos = i;
        }
    }
    return pos;
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Invalid Argument. Correct command | cargo run -- dry-run/organize file-path |");
        return ExitCode::from(1);
    }

    let flag = &args[1];
    let path = &args[2];

    println!("{} {}", flag, path);

    if flag != "organize" && flag != "dry-run" {
        println!("Invalid Flag. Correct command | cargo run -- dry-run/organize file-path |");
        return ExitCode::from(2);
    }

    let files = fs::read_dir(path);
    let files = match files {
        Ok(files) => files,

        //Error Handling Left
        Err(error) => {
            panic!("Invalid path {} ", error.kind());
        }
    };
    let mut files_vector: Vec<File> = Vec::new();

    for p in files {
        let file = match p {
            Ok(p) => p,

            //Error Handling Left
            Err(error) => {
                panic!("Couldn't Show {}", error.kind());
            }
        };

        files_vector.push(File::new(file.path()));
    }

    let mut images: Vec<File> = Vec::new();
    let mut documents: Vec<File> = Vec::new();
    let mut video: Vec<File> = Vec::new();
    let mut audio: Vec<File> = Vec::new();
    let mut archives: Vec<File> = Vec::new();
    let mut others: Vec<File> = Vec::new();

    let copy_files_vector = files_vector.clone();

    for file in files_vector {
        match file.file_type {
            Filetype::Images => images.push(file),
            Filetype::Documents => documents.push(file),
            Filetype::Video => video.push(file),
            Filetype::Audio => audio.push(file),
            Filetype::Archives => archives.push(file),
            Filetype::Others => others.push(file),
        }
    }
    if flag == "dry-run" {
        println!("Expected Output:");
        println!("{}", path);
        let file_classes = [
            (Filetype::Images, "Images", &images),
            (Filetype::Documents, "Documents", &documents),
            (Filetype::Video, "Video", &video),
            (Filetype::Audio, "Audio", &audio),
            (Filetype::Archives, "Archives", &archives),
            (Filetype::Others, "Others", &others),
        ];

        for (_, folder_name, files) in file_classes.iter() {
            match files.is_empty() {
                false => {
                    println!("  |--{}", folder_name);
                    for file in *files {
                        println!("     |--{}", file.name);
                    }
                }
                true => {}
            }
        }
        return ExitCode::from(0);
    }

    if flag == "organize" {
        println!("Organizing files in: {}", path);
        let file_classes = [
            (Filetype::Images, "Images", &images),
            (Filetype::Documents, "Documents", &documents),
            (Filetype::Video, "Video", &video),
            (Filetype::Audio, "Audio", &audio),
            (Filetype::Archives, "Archives", &archives),
            (Filetype::Others, "Others", &others),
        ];

        for (_, folder_name, files) in file_classes.iter() {
            if !files.is_empty() {
                match fs::create_dir_all(format!("{}/{}", path, folder_name)) {
                    Ok(_) => {
                        println!("Created directory: {}/", folder_name);
                    }
                    Err(error) => panic!(
                        "Failed to create {} directory: {}",
                        folder_name,
                        error.kind()
                    ),
                }
            }
        }

        let length = copy_files_vector.len();

        for file in copy_files_vector {
            let folder_name = match file.file_type {
                Filetype::Images => "Images",
                Filetype::Documents => "Documents",
                Filetype::Video => "Video",
                Filetype::Audio => "Audio",
                Filetype::Archives => "Archives",
                Filetype::Others => "Others",
            };
            let new_path = format!("{}/{}/{}", path, folder_name, file.name);
            match fs::rename(&file.path, &new_path) {
                Ok(_) => println!("Moved {} to {}", file.name, new_path),
                Err(error) => panic!("Failed to move {}: {}", file.name, error.kind()),
            }
        }

        println!("Moved {} files.", length);

        return ExitCode::from(0);
    }
    ExitCode::from(0)
}
