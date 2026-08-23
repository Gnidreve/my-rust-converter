use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // OUT_DIR liegt tief verschachtelt: target/debug/build/<pkg>-<hash>/out
    // 3 Ebenen hoch -> target/debug/ (bzw. target/release/)
    let profile_dir = out_dir.ancestors().nth(3).unwrap();

    let src = PathBuf::from("assets/ffmpeg.exe");
    let dest = profile_dir.join("ffmpeg.exe");

    if src.exists() {
        fs::copy(&src, &dest).expect("Konnte ffmpeg.exe nicht kopieren");
    }

    // Nur neu kopieren, wenn sich die Quelldatei ändert
    println!("cargo:rerun-if-changed=assets/ffmpeg.exe");
}