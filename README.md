# Hack youtube video limit

## Description

This is an advanced project to bypass the youtube video limit of 10 videos per day.

The python moviepy library can be used to create a video with the desired number of videos. In-fact it is the most popular method and I found lots of solution on the internet using moviepy. As well as ChatGPT also suggests to use moviepy to concatenate the videos.

However, I found that when using Rust, the process is **2040 times faster**, benchmarked against 84 videos of variable audio-bit rates with total length of 6 hours.

Using ffmpeg to concatenate the videos with variable bitrate audio streams is not possible, as the audio streams are not concatenated correctly.

To prove this, you can have a high-bit-rate audio stream in the first video, and a low-bit-rate audio stream in the second video. When concatenated, the audio stream of the second video will be played at the high-bit-rate of the first video, or the audio stream of the first video will be played at the low-bit-rate of the second video.

The Rust code in this project will concatenate the videos correctly, and the audio streams will be played at the correct bit-rate.

This bypasses the youtube video limit of 10 videos per day, as the videos are concatenated into one video but viewed as separate videos.

PS: a small scrapper also written for scrapping a source once it has been saved as an offline page (as the page is auth protected and can't be accessed directly + the source is not available in the page source immediately as it works with hydration and is not using SSR). This could be done with puppeteer but I am not using it and encourage you to use it if you want to scrap the source directly. I do not want to pay for proxies and I am not interested in using the free ones.

## Run

### To scrape the source if required

    ```bash
        # To scrape the source => will make folders from scraped website
        cargo run --release --bin make_folders
    ```

### Housekeeping

    ```bash
        # To make the playlists => will create playlist.txt
        cargo run --release --bin create_playlist
    ```

    ```bash
        # To delete unwated cache files => will delete unwanted files
        python delete_unwanted.py
    ```

### To reencode the videos for preventing Non-monotonic DTS errors while concatenating

    ```bash
        # To reencode the videos => will reencode the videos
        cargo run --release --bin arrange_improv
    ```

### To concatenate the videos

    ```bash
        find . -type f -name "._*" -delete
    ```

    ```bash
        # To concatenate the videos => will create combined_video.mp4
        cargo run --release --bin combine_videos
    ```

## Note

Direct way to delete unwated files

    ```bash
        find . -type f -name "._*" -delete
    ```

    offcourse to delete all .mp4 files (warning: this will delete all mp4 files)

    ```bash
        find . -type f -name "*.mp4" -delete
    ```

    ```bash
        # To prepare the support material => will delete all files except for support material
        python prepare_support_material.py
    ```

Direct bash command to make folder.txt

    ```bash
        find . -maxdepth 1 -type d -regex '\./[0-9].*' | sed 's/.\///' | sort > folders.txt
    ```

To copy files to the base directory and rename them, in case they have already been reencoded (rarity)

    ```bash
        # To replace the original files with reencoded files => will reencode the files
        # To pre-copy video files => will copy the video files to the output folder
        cargo run --release --bin arrange_videos
    ```

To make rust executable

    ```bash
        cargo build --release --bin arrange_improv && cp target/release/arrange_improv .
    ```

## Experimentation

Try standalone combine_videos.py instead of the rust combine videos

    ```bash
        # To concatenate the videos => will create combined_video.mp4
        find . -type f -name "._*" -delete
        python combine_videos.py
    ```

## Implications

- This bypasses the youtube video limit of 10 videos per day, as the videos are concatenated into one video but viewed as separate videos.
- The video can be uploaded to youtube, and the video limit is bypassed.
- The playlist is generated to view the videos separately.
- The playlist can be used as videojumper on youtube or on seperate website with custom video player (or even simple html video player) to give illusion of seperate videos.

Author: [Sagar Yadav](https://linkedin.com/in/sagaryadav)

Copyright: 2024 Sagar Yadav

All rights reserved.
The code cannot be used without the permission of the author.
