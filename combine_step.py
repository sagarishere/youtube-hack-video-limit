import os
from moviepy.editor import VideoFileClip, concatenate_videoclips

def get_sorted_mp4_files():
    # Get all .mp4 files in the current directory
    files = [f for f in os.listdir() if f.endswith('.mp4')]
    # Sort files by name
    files.sort()
    return files

def concatenate_last_two_videos():
    files = get_sorted_mp4_files()
    
    if len(files) < 2:
        print("Not enough videos to concatenate.")
        return
    
    # Get the last two video files
    last_file = files[-1]
    second_last_file = files[-2]
    
    # Load the video files
    clip1 = VideoFileClip(second_last_file)
    clip2 = VideoFileClip(last_file)
    
    # Concatenate the videos
    combined = concatenate_videoclips([clip1, clip2], method="compose")
    
    # Save the combined video as the second last file
    combined_filename = second_last_file
    combined.write_videofile(
        combined_filename,
        codec='libx264',
        temp_audiofile='temp-audio.m4a',
        remove_temp=True,
        audio_codec='aac',
        preset='ultrafast',  # Use a faster preset
        threads=4            # Use multiple threads if available
    )
    
    # Remove the last video file
    os.remove(last_file)
    
    print(f"Concatenated {second_last_file} and {last_file} into {combined_filename}.")

if __name__ == "__main__":
    concatenate_last_two_videos()
