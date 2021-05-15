mod util;
use colored::*;
use sha2::{self, Digest};
use std::io;
use std::{fs, io::Write};

use std::env;
fn panico(content: &str) {
    println!("{}", content.red());
    std::process::exit(1);
}
fn flush_output() {
    let _ = io::stdout().flush();
}

fn free<T>(_a: T) {}
fn main() {
    println!(
        "{}'s {} is {} by {}!\n",
        "yxqsnz".yellow(),
        "Console-vid-player".cyan(),
        "powered".bold(),
        "viu".green()
    );

    if !util::viu_installed() {
        panico("You need install viu!: https://github.com/atanunq/viu")
    }
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panico("Please tell me a file!");
    }
    if !std::path::Path::new(args[1].as_str()).exists() {
        panico(
            format!(
                "The file '{}' does {} exist.",
                args[1].cyan(),
                "not".yellow()
            )
            .as_str(),
        )
    }
    print!(
        "{} {}'s {}... ",
        "Generating".purple(),
        args[1].cyan(),
        "hash".yellow()
    );
    flush_output();
    let mut sha256 = sha2::Sha256::new();
    let mut file = fs::File::open(args[1].as_str()).unwrap();
    let _ = io::copy(&mut file, &mut sha256);
    let raw_hash = sha256.finalize();
    let hash = format!("{:x}", raw_hash);
    println!("{}", "done".green());
    free(raw_hash);
    free(file);
    let file_path = std::path::PathBuf::from(args[1].as_str());
    let absolute_file_path__ = file_path.canonicalize().unwrap();
    let absolute_file_path = absolute_file_path__.to_str().unwrap();
    println!("{} {}: ", "Processing".green(), args[1].cyan());
    let (frames_path, sound_path) = util::create_frames_dir(&hash);
    let file_props = util::read_props(&absolute_file_path.to_string());
    print!("  {} {}... ", "Extracting".green(), "Sound".cyan());
    flush_output();
    util::convert_video_to_audio(&absolute_file_path.to_string(), &sound_path);
    println!("{}", "Done".green());
    print!("  {} {}... ", "Extracting".green(), "Frames".yellow());
    flush_output();
    if !util::ignore_pf(
        file_props.streams[0].nb_frames.parse().unwrap(),
        &frames_path,
    ) {
        util::extract_frames(absolute_file_path, &frames_path);
        println!("{}", "Done".green());
    } else {
        println!("{}", "Skipped".yellow());
    }
    free(args);
    util::play_sound(&sound_path);

    util::play(&frames_path);
}
