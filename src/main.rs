use mp4_merge::join_files;
use std::fs;

fn main() {
    // Directory containing the video files
    let directory = "./";

    // Collect all .mp4 files in the directory
    let mut video_files: Vec<String> = fs::read_dir(directory)
        .expect("Failed to read directory")
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

    // Sort files alphabetically
    video_files.sort();

    println!("Files to concatenate: {:?}", video_files);

    // Ensure there are files to merge
    if video_files.is_empty() {
        println!("No video files found in the directory.");
        return;
    }

    // Output file path
    let output_file = format!("{}combined_video.mp4", directory);

    // Merge video files
    match join_files(&video_files, &output_file, |progress| {
        println!("Merging... {:.2}%", progress * 100.0);
    }) {
        Ok(_) => println!("Video has been successfully created at: {}", output_file),
        Err(e) => println!("Failed to merge video files: {}", e),
    }
}
