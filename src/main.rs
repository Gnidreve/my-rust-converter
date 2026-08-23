mod view;
mod update;
use update::update;
use update::subscription;
use view::view;
use std::path::{Path, PathBuf};

const VIDEO_FORMATS: [&str; 3] = ["mp4", "MOV", "AVI"];
const AUDIO_FORMATS: [&str; 4] = ["mp3", "wav", "flac", "aac"];
const IMAGE_FORMATS: [&str; 4] = ["png", "jpeg", "jpg", "WebP"];

fn main() -> iced::Result {

    // Loading the Icon for the Application (fest ins Binary eingebettet, kein Datei-Zugriff zur Laufzeit)
    let app_icon = iced::window::icon::from_file_data(include_bytes!("../assets/app-icon.jpg"), None)
        .expect("Couldn`t load the Application-Logo");

    // Starting the Application
    iced::application(State::default, update, view)
        .title("My Rust Converter")
        .window(iced::window::Settings {
            icon: Some(app_icon),
            ..Default::default()
        })
        .subscription(subscription)
        .run()
}

//Globaler State
#[derive(Default)]
struct State {
    selected_file: Option<PathBuf>,
    selected_format: Option<&'static str>,
    converted_file: Option<PathBuf>,
}

// Gibt nur die Formate zurück, die als Konvertierungsziel für die gegebene Datei Sinn ergeben
// (gleiche Kategorie wie die Eingabedatei, ohne deren aktuelles Format selbst)
pub fn available_formats(input_path: &Path) -> Vec<&'static str> {
    let current_ext = input_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    let category: &[&str] = if VIDEO_FORMATS.iter().any(|f| f.eq_ignore_ascii_case(current_ext)) {
        &VIDEO_FORMATS
    } else if AUDIO_FORMATS.iter().any(|f| f.eq_ignore_ascii_case(current_ext)) {
        &AUDIO_FORMATS
    } else if IMAGE_FORMATS.iter().any(|f| f.eq_ignore_ascii_case(current_ext)) {
        &IMAGE_FORMATS
    } else {
        &[]
    };

    category
        .iter()
        .copied()
        .filter(|format| !format.eq_ignore_ascii_case(current_ext))
        .collect()
}



pub fn ffmpeg_path() -> PathBuf {
    let mut path = std::env::current_exe()
        .expect("Eigener Pfad konnte nicht ermittelt werden");
    path.pop(); // Ordner der exe (nicht die exe-Datei selbst)
    path.push("ffmpeg.exe");
    path
}



