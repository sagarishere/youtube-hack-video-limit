use mp4::Mp4Reader;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::time::Duration;

fn format_duration(dur: Duration) -> String {
    let total_seconds = dur.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn get_mp4_duration(file_path: &Path) -> Result<Duration, mp4::Error> {
    let file = File::open(file_path)?;
    let size = file.metadata()?.len();
    let reader = BufReader::new(file);
    let mp4 = Mp4Reader::read_header(reader, size)?;
    Ok(mp4.duration())
}

fn get_mp4_files_sorted(folders: Vec<String>) -> std::io::Result<()> {
    let mut playlist = BufWriter::new(File::create("playlist.txt")?);
    writeln!(playlist, "⭐️ Contents ⭐️")?;

    let mut course_time_passed = Duration::new(0, 0);

    for folder in folders {
        writeln!(playlist, "\n📂 {}", folder)?;

        let folder_path = Path::new(&folder);
        let mut files: Vec<_> = fs::read_dir(folder_path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() && path.extension()? == "mp4" {
                    Some(entry.path())
                } else {
                    None
                }
            })
            .collect();

        files.sort();

        let mut course_time = course_time_passed;

        for file in files {
            match get_mp4_duration(&file) {
                Ok(duration_secs) => {
                    writeln!(
                        playlist,
                        "⌨️ ({}) {}",
                        format_duration(course_time),
                        (file
                            .file_name()
                            .unwrap()
                            .to_os_string()
                            .into_string()
                            .unwrap())
                        .strip_suffix(".mp4")
                        .unwrap()
                    )?;
                    course_time += duration_secs;
                }
                Err(e) => {
                    eprintln!("Error reading duration for file {:?}: {}", file, e);
                }
            }
        }

        course_time_passed = course_time;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    loop {
        let fileresult = File::open("folders.txt");
        let file = match fileresult {
            Ok(file) => file,
            Err(e) => {
                println!("Error: {}", e);
                println!("Creating folders.txt file");
                makefoldertxt();
                continue;
            }
        };
        let reader = BufReader::new(file);
        let folders: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

        if let Err(e) = get_mp4_files_sorted(folders) {
            eprintln!("Error: {}", e);
        }

        break;
    }

    Ok(())
}

fn makefoldertxt() {
    let output = Command::new("bash")
        .arg("-c")
        .arg("find . -maxdepth 1 -type d -regex '\\./[0-9].*' | sed 's/\\.\\/\\(.*\\)/\\1/' | sort > folders.txt")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("Status: {}", output.status);
    if stdout.trim().len() > 0 {
        println!("stdout: {}", stdout);
    }
    if stderr.trim().len() > 0 {
        println!("stderr: {}", stderr);
    }
}
