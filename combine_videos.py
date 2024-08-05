import os
from pathlib import Path
from moviepy.editor import VideoFileClip
import shutil

def get_folders_sorted(base_dir):
    folders = [f for f in os.listdir(base_dir) if os.path.isdir(os.path.join(base_dir, f))]
    folders.sort()
    return folders

def get_mp4_files_sorted(base_dir, folders):
    mp4_files = []
    for folder in folders:
        folder_path = os.path.join(base_dir, folder)
        files = [f for f in os.listdir(folder_path) if f.endswith('.mp4')]
        files.sort()
        for file in files:
            mp4_files.append((folder, file))
    return mp4_files

def reencode_video(input_path, output_path):
    clip = VideoFileClip(input_path)
    clip.write_videofile(output_path, codec='libx264', audio_codec='aac', fps=30)

def reencode_files_parallel(base_dir, mp4_files):
    for folder, file_name in mp4_files:
        input_path = os.path.join(base_dir, folder, file_name)
        temp_output = input_path.replace('.mp4', '.tmp.mp4')
        reencode_video(input_path, temp_output)
        os.remove(input_path)
        shutil.move(temp_output, input_path)
        print(f"Re-encoded: {input_path}")

def rename_files_sequential(base_dir, mp4_files):
    renamed_files = []
    for index, (folder, file_name) in enumerate(mp4_files):
        old_path = os.path.join(base_dir, folder, file_name)
        new_file_name = f"{index + 1:03}_{file_name}"
        new_path = os.path.join(base_dir, new_file_name)
        os.rename(old_path, new_path)
        renamed_files.append(new_file_name)
        print(f"Renamed: {old_path} to {new_path}")
    return renamed_files

def create_concat_file(renamed_files):
    with open('ffmpeg_concat_list.txt', 'w') as file:
        for file_name in renamed_files:
            file.write(f"file '{file_name}'\n")

def write_to_folders_txt(folders):
    with open('folders.txt', 'w') as file:
        for folder in folders:
            file.write(f"{folder}\n")

def main():
    base_dir = Path(__file__).parent
    folders = get_folders_sorted(base_dir)
    write_to_folders_txt(folders)
    mp4_files = get_mp4_files_sorted(base_dir, folders)
    reencode_files_parallel(base_dir, mp4_files)
    renamed_files = rename_files_sequential(base_dir, mp4_files)
    create_concat_file(renamed_files)

if __name__ == "__main__":
    main()
