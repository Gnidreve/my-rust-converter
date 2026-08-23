use crate::{State, ffmpeg_path}; // Global State from main.
use crate::{VIDEO_FORMATS, AUDIO_FORMATS, IMAGE_FORMATS};
use rfd::FileDialog; // rfd crate for the file-picker-dialog
use std::process::Command; // core crate for cli interaction
use std::path::PathBuf;
use iced::{Event, Subscription, window,};


#[derive(Debug, Clone)]
pub enum Action {
    PickFile,
    ConvertFile,
    FileDropped(PathBuf), // Ausgelöst, wenn eine Datei per Drag & Drop ins Fenster gezogen wird
	FormatSelected(&'static str),
	SaveAs // Ausgelöst durch den Download-Button, öffnet den "Speichern unter"-Dialog
}


pub fn update(state: &mut State, message: Action) {
    match message {

		// Logic for the file-picker-dialog
        Action::PickFile => {
	
            if let Some(path) = FileDialog::new()
                .add_filter("All Files", &["*"])
                .add_filter("Videos", &VIDEO_FORMATS)
                .add_filter("Audios", &AUDIO_FORMATS)
                .add_filter("Images", &IMAGE_FORMATS)
                .pick_file() {
                state.selected_file = Some(path);
				// Neue Datei -> altes Format/alte Konvertierung sind nicht mehr gültig
				state.selected_format = None;
				state.converted_file = None;
            }
            println!("Added File..");
        }

		// Datei per Drag & Drop fallen gelassen -> automatisch als ausgewählt übernehmen
		Action::FileDropped(path) => {
			state.selected_file = Some(path);
			state.selected_format = None;
			state.converted_file = None;
		}

		// Logic for the selection of selected_format
		Action::FormatSelected(format) => {
            state.selected_format = Some(format);
			state.converted_file = None; // neues Zielformat -> alte Konvertierung verwerfen
        }
		

		// Logic for converting the file 
		Action::ConvertFile => {
			println!("Converting file..");

            // Checking for provided file and using it as input_path
            if let Some(input_path) = &state.selected_file {
                // Validating the format 
                if let Some(format) = state.selected_format {
                    // Concatenating the format to the filename for the output
                    let output_path = input_path.with_extension(format);
                    let output = Command::new(ffmpeg_path())
                    .arg("-i")
                    .arg(input_path)
                    .arg("-y") // Zieldatei ohne Rückfrage überschreiben (sonst hängt der Prozess)
                    .arg(&output_path)
                    .output()
                    .expect("ffmpeg konnte nicht gestartet werden");

                    if output.status.success() {
                        state.converted_file = Some(output_path);
                        println!("Converting finished!");
                    } else {
                        state.converted_file = None;
                        println!("ffmpeg-Fehler: {}", String::from_utf8_lossy(&output.stderr));
                    }
                }
            }
        }

		// Speichert die konvertierte Datei an einem vom Nutzer gewählten Ort ("Speichern unter")
		Action::SaveAs => {
            if let Some(converted_file) = &state.converted_file {
                let default_name = converted_file
                    .file_name()
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_default();

                if let Some(destination) = FileDialog::new().set_file_name(default_name).save_file() {
                    std::fs::copy(converted_file, destination).expect("Datei konnte nicht gespeichert werden");
                }
            }
        }
    }
}

// Lauscht global (fensterweit) auf System-Events und filtert auf "Datei gedroppt".
// Läuft unabhängig von Klicks/Buttons im Hintergrund, solange die App offen ist.
// Listens to background event while the app is open
pub fn subscription(_state: &State) -> Subscription<Action> {
    iced::event::listen_with(|event, _status, _window| {
        if let Event::Window(window::Event::FileDropped(path)) = event {
            Some(Action::FileDropped(path))
        } else {
            None
        }
    })
}
