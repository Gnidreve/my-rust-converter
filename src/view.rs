use crate::State; // Global State from main.rs
use crate::available_formats;
use crate::update::Action; // Actions from update.rs

use iced::{
    Element, 
    Alignment
}; 

// iced widget and helpers
use iced::widget::{
    button, 
    center, column, 
    text, 
    pick_list,
};

// Rendering loop (gets called every frame)
pub fn view(state: &State) -> Element<'_, Action> {
    // The Main Frame for rendering stuff in the window
    let mut content: iced::widget::Column<'_, Action> = column![
        // Here the content.pushes get inserted conditionally
    ].spacing(12).align_x(Alignment::Center);

    // Conditional rendering into content (if a file is NOT selected)
    if state.selected_file.is_none() {
        content = content.push(button("Add file").on_press(Action::PickFile))
    }

    // Conditional rendering into content (if a file is selected)
    if let Some(path) = &state.selected_file {
        content = content.push(text(path.display().to_string()));
        // Nur die Formate anzeigen, die für diese Datei als Ziel Sinn ergeben
        let formats = available_formats(path);
        content = content.push(pick_list(formats, state.selected_format, Action::FormatSelected).placeholder("Select output format"));
    }

    if let Some(_format) = &state.selected_format {
    content = content.push(button("Convert File").on_press(Action::ConvertFile));
    }

    // Nach erfolgreicher Konvertierung: Download-Button anzeigen
    if state.converted_file.is_some() {
        content = content.push(text("Conversion complete!"));
        content = content.push(button("Download").on_press(Action::SaveAs));
    }

    // Renders all content blocks in the window (centered)
    center(content).into()
}
