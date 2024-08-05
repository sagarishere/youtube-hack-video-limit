import os
import sys
import shutil

def delete_all_files_except_self():
    """Deletes all files in the current directory except for the script itself."""
    # Get the current directory
    current_dir = os.getcwd()
    
    # List all files in the current directory
    files = os.listdir(current_dir)
    
    # Filter out the script file (excluding itself)
    script_name = os.path.basename(sys.argv[0])
    files_to_delete = [f for f in files if f != script_name]
    
    # Delete each file
    for file in files_to_delete:
        file_path = os.path.join(current_dir, file)
        if os.path.isfile(file_path):
            os.remove(file_path)

def get_non_numeric_dirs():
    """Returns a list of directories in the current directory whose names do not start with a digit."""
    current_dir = os.getcwd()
    dirs = os.listdir(current_dir)
    non_numeric_dirs = [d for d in dirs if not d[0].isdigit()]
    return non_numeric_dirs

def delete_directories(directories):
    """Deletes all directories listed in the input list."""
    for dir_name in directories:
        dir_path = os.path.join(os.getcwd(), dir_name)
        if os.path.isdir(dir_path):
            shutil.rmtree(dir_path)

if __name__ == "__main__":
    delete_all_files_except_self()
    non_numeric_dirs = get_non_numeric_dirs()
    delete_directories(non_numeric_dirs)