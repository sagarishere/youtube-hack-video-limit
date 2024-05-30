import os
import shutil
import subprocess

def get_folders_sorted():
    """Get a list of all folders in the current directory, sorted lexically."""
    current_dir = os.getcwd()
    folders = [f for f in os.listdir(current_dir) if os.path.isdir(os.path.join(current_dir, f))]
    folders.sort()
    return folders

def get_mp4_files_sorted(folders):
    """Get a list of .mp4 files from the given folders, sorted by directory and lexically."""
    mp4_files = []
    for folder in folders:
        folder_path = os.path.join(os.getcwd(), folder)
        files = [f for f in os.listdir(folder_path) if f.endswith('.mp4')]
        files.sort()
        mp4_files.extend([(folder, f) for f in files])
    return mp4_files

def copy_and_rename_files(mp4_files):
    """Copy .mp4 files to the current directory and rename them."""
    current_dir = os.getcwd()
    renamed_files = []
    for index, (folder, file_name) in enumerate(mp4_files, start=1):
        old_path = os.path.join(current_dir, folder, file_name)
        new_file_name = f"{index:02d}{file_name[file_name.find(' '):]}"  # Renaming logic
        new_path = os.path.join(current_dir, new_file_name)
        shutil.copy(old_path, new_path)
        renamed_files.append(new_file_name)
        print(f"Copied and renamed: {old_path} to {new_path}")
    return renamed_files

def create_ffmpeg_concat_file(renamed_files):
    """Create a temporary file for ffmpeg concatenation."""
    with open('ffmpeg_concat_list.txt', 'w') as f:
        for file_name in renamed_files:
            f.write(f"file '{file_name}'\n")

def main():
    folders = get_folders_sorted()
    write_to_folders_txt(folders)
    mp4_files = get_mp4_files_sorted(folders)
    renamed_files = copy_and_rename_files(mp4_files)
    create_ffmpeg_concat_file(renamed_files)

def write_to_folders_txt(folders):
    with open('folders.txt', 'w') as f:
        for folder in folders:
            f.write(f"{folder}\n")

if __name__ == "__main__":
    main()
