import os

def delete_special_mp4_files(directory):
    """Recursively delete files that start with '._' and end with '.mp4'."""
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.startswith("._") and file.endswith(".mp4"):
                file_path = os.path.join(root, file)
                try:
                    os.remove(file_path)
                    print(f"Deleted: {file_path}")
                except Exception as e:
                    print(f"Failed to delete {file_path}: {e}")

def main():
    current_dir = os.getcwd()  # or set the directory you want to start from
    delete_special_mp4_files(current_dir)

if __name__ == "__main__":
    main()