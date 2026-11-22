use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "yt_downloader")]
#[command(about = "Download YouTube videos or audio using yt-dlp")]
struct Args {
    url: String,
    #[arg(short, long)]
    audio: bool,
}

fn main() {
    let args = Args::parse();
    let mut cmd = Command::new("yt-dlp");

    if args.audio {
        cmd.args(["-x", "--audio-format", "mp3", &args.url]);
    } else {
        cmd.args(["-f", "bv*+ba/b", "--merge-output-format", "mp4", &args.url]);
    }

    println!("Starting download...");

    match cmd.status() {
        Ok(status) if status.success() => {
            println!("Download completed successfully.");
        }
        Ok(status) => {
            eprintln!("yt-dlp exited with error code: {:?}", status.code());
        }
        Err(e) => {
            eprintln!("Failed to run yt-dlp: {e}");
            eprintln!("Make sure yt-dlp is installed and in your PATH.");
        }
    }
}
