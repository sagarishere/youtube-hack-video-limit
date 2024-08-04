use rayon::prelude::*;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn get_folders_sorted() -> Vec<String> {
    let current_dir = env::current_dir().unwrap();
    let mut folders: Vec<String> = fs::read_dir(current_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                Some(path.file_name().unwrap().to_str().unwrap().to_string())
            } else {
                None
            }
        })
        .collect();
    folders.sort();
    folders
}

fn get_mp4_files_sorted(folders: Vec<String>) -> Vec<(String, String)> {
    let mut mp4_files = Vec::new();
    for folder in folders {
        let folder_path = env::current_dir().unwrap().join(&folder);
        let mut files: Vec<String> = fs::read_dir(&folder_path)
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.unwrap();
                let file_name = entry.file_name().into_string().unwrap();
                if file_name.ends_with(".mp4") {
                    Some(file_name)
                } else {
                    None
                }
            })
            .collect();

        for file in &mut files {
            if !file.chars().nth(0).unwrap().is_digit(10)
                || !file.chars().nth(1).unwrap().is_digit(10)
            {
                let new_file = format!("0{}", file);
                let old_path = folder_path.join(&file);
                let new_path = folder_path.join(&new_file);
                fs::rename(&old_path, &new_path).unwrap();
                *file = new_file;
            }
        }
        files.sort();
        for file in files {
            mp4_files.push((folder.clone(), file));
        }
    }
    mp4_files
}

fn reencode_video(input_file: &str) -> io::Result<()> {
    let temp_output = format!("{}.tmp.mp4", input_file);
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_file)
        .arg("-vf")
        .arg("fps=30")
        .arg("-c:v")
        .arg("libx264")
        .arg("-crf")
        .arg("23")
        .arg("-preset")
        .arg("fast")
        .arg("-c:a")
        .arg("aac")
        .arg("-b:a")
        .arg("192k")
        .arg(&temp_output)
        .status();

    match status {
        Ok(status) if status.success() => {
            fs::rename(&temp_output, input_file)?; // Replace original file
            Ok(())
        }
        Ok(status) => {
            eprintln!(
                "FFmpeg failed to re-encode {} with exit code: {}",
                input_file, status
            );
            Err(io::Error::new(
                io::ErrorKind::Other,
                "FFmpeg re-encode failed",
            ))
        }
        Err(e) => {
            eprintln!("Failed to execute FFmpeg for re-encoding: {}", e);
            Err(e)
        }
    }
}

fn reencode_files_parallel(mp4_files: &[(String, String)]) {
    let current_dir = env::current_dir().unwrap();
    mp4_files.par_iter().for_each(|(folder, file_name)| {
        let old_path = current_dir.join(folder).join(file_name);
        reencode_video(old_path.to_str().unwrap()).unwrap();
    });
}

fn rename_files_sequential(mp4_files: Vec<(String, String)>) -> Vec<String> {
    let current_dir = env::current_dir().unwrap();
    let mut renamed_files = Vec::new();

    for (index, (folder, file_name)) in mp4_files.into_iter().enumerate() {
        let old_path = current_dir.join(&folder).join(&file_name);
        let new_file_name = format!(
            "{:03}{}",
            index + 1,
            &file_name[file_name.find(' ').unwrap_or(0)..]
        );
        let new_path = current_dir.join(&new_file_name);
        fs::copy(&old_path, &new_path).unwrap();
        renamed_files.push(new_file_name.clone());
        println!("Renamed: {:?} to {:?}", old_path, new_path);
    }
    renamed_files
}

fn create_ffmpeg_concat_file(renamed_files: Vec<String>) {
    let mut file = fs::File::create("ffmpeg_concat_list.txt").unwrap();
    for file_name in renamed_files {
        writeln!(file, "file '{}'", file_name).unwrap();
    }
}

fn write_to_folders_txt(folders: Vec<String>) {
    let mut file = fs::File::create("folders.txt").unwrap();
    for folder in folders {
        writeln!(file, "{}", folder).unwrap();
    }
}

fn main() {
    let folders = get_folders_sorted();
    write_to_folders_txt(folders.clone());
    let mp4_files = get_mp4_files_sorted(folders);
    reencode_files_parallel(&mp4_files);
    let renamed_files = rename_files_sequential(mp4_files);
    create_ffmpeg_concat_file(renamed_files);
}
