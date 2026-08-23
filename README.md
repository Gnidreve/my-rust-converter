# My Rust Converter

This is my first ever project in rust with ffmpeg. i thought this would be a good starting point for learning the rust language.

Every line of code in this project was typed by hand because i'am trying to learn something new, not trying to get something done.

You can download it on the github releases (currently for windows only)

## What this project does

- Get a file via a file-picker dialog
- OR get a file via drag-and-drop
- Calculate the possible output formats from the input file
- Perform a convertion with ffmpeg
- Make the output file downloadable

## Things i needed to do for this project:

- Open a Window with iced
- Setting an Application logo and title
- Building a minimal UI
- Implementing EventHandling
- Using subscription to listen to dropped files
-
- Getting the input file from the path buffer to the ffmpeg cli
- Converting the input file to the given output format
- Making the output file downloadable

## Rust-Dependencys

- iced
- rfd::FileDialog
- std::process::Command
- std::path::PathBuf

## Sources:

docs.rs
