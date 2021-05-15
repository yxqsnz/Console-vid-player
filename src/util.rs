use colored::*;
use serde::Deserialize;
use serde_json::from_str as parse_json;
use std::fs;
use std::io::{stdout, Write};
use std::process::Command;
use play::play as _play_s;
pub fn viu_installed() -> bool {
    let x = Command::new("viu").output();
    x.is_ok()
}
#[derive(Deserialize)]
pub struct Stream {
    pub duration: String,
    pub nb_frames: String,
    pub r_frame_rate: String,
}
#[derive(Deserialize)]
pub struct VideoProps {
    pub streams: Vec<Stream>,
}
#[derive(Debug)]
pub struct FailedToExtractError;

pub fn ignore_pf(nbf: usize, frames_path: &str) -> bool {
    let paths = fs::read_dir(frames_path).unwrap();
    paths.count() == nbf
}
pub fn play_sound(sound_path: &str) {
   
   std::thread::spawn({
       let _sound_path = sound_path.to_string(); 
       move || {
        _play_s(format!("{}/sound.mp3", _sound_path)).unwrap();
   }}); 
}
pub fn play(frames_path: &str) {
    let paths = fs::read_dir(frames_path).unwrap();
    let paths_count = paths.count();
    for i in 0..paths_count {
        let _ = Command::new("viu")
            .arg(format!("{}/frame.{}.png", frames_path, i).as_str())
            .spawn()
            .unwrap()
            .wait();
        print!(
            "{}\r",
            format!("Frame {} of {}", i, paths_count).truecolor(98, 114, 164)
        );
        let _ = stdout().flush();
    }
}
pub fn convert_video_to_audio(file_path: &str, sound_path: &str) {
    // ffmpeg -i sample.avi -q:a 0 -map a sample.mp3
    let _ = Command::new("ffmpeg")
        .args(&[
            "-i",
            file_path,
            "-q:a",
            "0",
            "-map",
            "a",
            format!("{}/sound.ogg", sound_path).as_str(),
        ])
        .output()
        .unwrap();
}
pub fn extract_frames(file_path: &str, frames_path: &str) {
    // ffmpeg -i ../bad_apple.mp4 frame.%d.png
    let _ = Command::new("ffmpeg")
        .args(&[
            "-i",
            file_path,
            format!("{}/frame.%d.png", frames_path).as_str(),
        ])
        .output()
        .unwrap();
}
pub fn read_props(file_path: &str) -> VideoProps {
    let command_output = Command::new("ffprobe")
        .args(&[
            "-i",
            file_path,
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_streams",
        ])
        .output()
        .unwrap();
    let command_stdout_out = std::str::from_utf8(&command_output.stdout).unwrap();

    let result: VideoProps = parse_json(command_stdout_out).unwrap();

    drop(command_output);
    result
}
pub fn create_frames_dir(video_hash: &str) -> (String, String) {
    let home_dir = dirs::home_dir().unwrap();
    let frames_path = format!(
        "{}/.cvp/cache/video/{}/frames",
        home_dir.display(),
        video_hash
    );
    let sound_path = format!(
        "{}/.cvp/cache/video/{}/sound",
        home_dir.display(),
        video_hash
    );

    let result = fs::create_dir_all(frames_path.clone());
    let _ = fs::create_dir_all(sound_path.clone());
    match result {
        Ok(_) => (),
        Err(e) => panic!("{:#?}", e),
    };
    (frames_path, sound_path)
}
