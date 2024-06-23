import os
from moviepy.editor import VideoFileClip, concatenate_videoclips

def get_sorted_mp4_files():
    """
    A function to get all .mp4 files in the current directory and sort them by name.
    """
    # Get all .mp4 files in the current directory
    files = [f for f in os.listdir() if f.endswith('.mp4')]
    # Sort files by name
    files.sort()
    return files

def concatenate_videos(output_filename='combined_video.mp4'):
    """
    Concatenates all the .mp4 files in the current directory into a single video file.
    
    Parameters:
        output_filename (str, optional): The name of the output video file. Defaults to 'combined_video.mp4'.
    
    Returns:
        None
    
    Prints:
        - "No .mp4 files found in the current directory." if no .mp4 files are found in the current directory.
    """
    files = get_sorted_mp4_files()
    
    if not files:
        print("No .mp4 files found in the current directory.")
        return
    
    # Start with the first video file
    combined = VideoFileClip(files[0])
    
    for file in files[1:]:
        next_clip = VideoFileClip(file)
        combined = concatenate_videoclips([combined, next_clip], method="compose")
    
    # Write the final combined video to the output file
    combined.write_videofile(output_filename, codec='libx264')

if __name__ == "__main__":
    concatenate_videos()
