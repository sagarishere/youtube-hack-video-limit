use rand::Rng;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() -> io::Result<()> {
    // Directory containing the video files
    let directory = "./";

    loop {
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

        // If only one video file is left, exit the loop
        if video_files.len() == 1 {
            println!("Only one video file remaining: {}", video_files[0]);
            break;
        }

        // Select a random consecutive pair of videos
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..video_files.len() - 1);

        let file1 = &video_files[random_index];
        let file2 = &video_files[random_index + 1];

        println!("Combining {} and {}", file1, file2);

        // Re-encode the selected video files
        let reencoded_file1 = format!("reencoded_{}.mp4", random_index);
        let reencoded_file2 = format!("reencoded_{}.mp4", random_index + 1);

        reencode_video(file1, &reencoded_file1)?;
        reencode_video(file2, &reencoded_file2)?;

        // Create a temporary text file with the list of the re-encoded video files
        let concat_list = "concat_list.txt";
        {
            let mut concat_file = File::create(concat_list)?;
            writeln!(concat_file, "file '{}'", reencoded_file1)?;
            writeln!(concat_file, "file '{}'", reencoded_file2)?;
        }

        // Output file path for the combined video
        let output_file = format!("combined_{}.mp4", random_index);

        // Concatenate the re-encoded video files using FFmpeg
        let status = Command::new("ffmpeg")
            .arg("-f")
            .arg("concat")
            .arg("-safe")
            .arg("0")
            .arg("-i")
            .arg(concat_list)
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
            .arg(&output_file)
            .status();

        match status {
            Ok(status) if status.success() => {
                println!(
                    "Successfully combined {} and {} into {}",
                    reencoded_file1, reencoded_file2, output_file
                );

                // Replace the first file with the combined output
                fs::rename(&output_file, file1)?;

                // Delete the second file
                fs::remove_file(file2)?;

                println!(
                    "Replaced {} with combined video and deleted {}",
                    file1, file2
                );
            }
            Ok(status) => {
                eprintln!("FFmpeg failed with exit code: {}", status);
            }
            Err(e) => {
                eprintln!("Failed to execute FFmpeg: {}", e);
            }
        }

        // Clean up temporary files
        fs::remove_file(concat_list)?;
        fs::remove_file(&reencoded_file1)?;
        fs::remove_file(&reencoded_file2)?;
    }

    Ok(())
}

fn reencode_video(input_file: &str, output_file: &str) -> io::Result<()> {
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
