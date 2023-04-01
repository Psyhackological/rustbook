use std::fs::File;
use std::io::{self, Read};
use std::process::Command;

fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

#[test]
fn test_output_against_lyrics_txt() {
    // Run the compiled binary and capture its output
    let output = Command::new("cargo")
        .args(&["run", "--quiet"])
        .output()
        .expect("Failed to execute the command");

    // Convert the output to a String
    let generated_lyrics = String::from_utf8(output.stdout).expect("Output contains invalid UTF-8");

    // Read the expected lyrics from 'lyrics/lyrics.txt'
    let expected_lyrics =
        read_file_to_string("lyrics/lyrics.txt").expect("Failed to read lyrics.txt");

    // Compare the generated lyrics with the expected lyrics
    assert_eq!(
        expected_lyrics, generated_lyrics,
        "Generated lyrics do not match the expected lyrics in 'lyrics/lyrics.txt'"
    );
}
