use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() -> io::Result<()> {
    // Directory containing the video files
    let directory = "./";

    // Collect all .mp4 files in the directory
    let mut video_files: Vec<String> = fs::read_dir(directory)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("mp4") {
                    path.to_str().map(|s| s.to_string())
                } else {
                    None
                }
            })
        })
        .collect();

    // Sort files alphabetically considering numeric values
    video_files.sort_by(|a, b| {
        let a_name = Path::new(a).file_name().unwrap().to_str().unwrap();
        let b_name = Path::new(b).file_name().unwrap().to_str().unwrap();

        let a_num = a_name.split_whitespace().next().unwrap_or("");
        let b_num = b_name.split_whitespace().next().unwrap_or("");

        let a_num: usize = a_num.parse().unwrap_or(0);
        let b_num: usize = b_num.parse().unwrap_or(0);

        a_num.cmp(&b_num)
    });

    println!("Files to concatenate: {:?}", video_files);

    // Check which files need re-encoding
    println!("Checking which files need re-encoding...");
    let reencode_flags: Vec<_> = video_files
        .par_iter()
        .map(|file| check_non_monotonous_dts(file).unwrap_or(false))
        .collect();

    // Re-encode files in parallel if necessary
    println!("Re-encoding files if necessary...");
    let reencode_progress = ProgressBar::new(video_files.len() as u64);
    reencode_progress.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    video_files
        .par_iter()
        .zip(reencode_flags.par_iter())
        .for_each(|(file, &needs_reencode)| {
            let reencoded_file = format!(
                "reencoded_{}",
                Path::new(file).file_name().unwrap().to_str().unwrap()
            );
            if needs_reencode {
                reencode_video(file, &reencoded_file).expect("Failed to re-encode video");
            } else {
                fs::copy(file, &reencoded_file).expect("Failed to copy video");
            }
            reencode_progress.inc(1);
        });
    reencode_progress.finish_with_message("Re-encoding complete");

    // Create a temporary text file with the list of the re-encoded video files
    println!("Creating concat list...");
    let concat_list = "concat_list.txt";
    {
        let mut concat_file = File::create(concat_list)?;
        for file in &video_files {
            let reencoded_file = format!(
                "reencoded_{}",
                Path::new(file).file_name().unwrap().to_str().unwrap()
            );
            writeln!(concat_file, "file '{}'", reencoded_file)?;
        }
    }

    // Output file path for the combined video
    let output_file = "combined_video.mp4";

    // Concatenate all the re-encoded video files using FFmpeg
    println!("Concatenating videos...");
    let status = Command::new("ffmpeg")
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(concat_list)
        .arg("-c")
        .arg("copy")
        .arg(&output_file)
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("Successfully combined videos into {}", output_file);
        }
        Ok(status) => {
            eprintln!("FFmpeg failed with exit code: {}", status);
        }
        Err(e) => {
            eprintln!("Failed to execute FFmpeg: {}", e);
        }
    }

    // Clean up temporary files
    println!("Cleaning up temporary files...");
    fs::remove_file(concat_list)?;
    video_files.iter().for_each(|file| {
        let reencoded_file = format!(
            "reencoded_{}",
            Path::new(file).file_name().unwrap().to_str().unwrap()
        );
        fs::remove_file(reencoded_file).expect("Failed to remove re-encoded file");
    });

    Ok(())
}

fn check_non_monotonous_dts(file: &str) -> io::Result<bool> {
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(file)
        .arg("-f")
        .arg("null")
        .arg("-")
        .output()?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    Ok(stderr.contains("Non-monotonous DTS"))
}

fn reencode_video(input_file: &str, output_file: &str) -> io::Result<()> {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_file)
        .arg("-c:v")
        .arg("libx264")
        .arg("-crf")
        .arg("23")
        .arg("-preset")
        .arg("veryfast")
        .arg(output_file)
        .status();

    match status {
        Ok(status) if status.success() => Ok(()),
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
