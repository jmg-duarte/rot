use clap::{App, Arg};

mod caesar;
mod io;

const ARG_NAME_INPUT_TEXT: &'static str = "text";
const ARG_NAME_ROTATION: &'static str = "rotation";
const ARG_NAME_DECIPHER: &'static str = "decipher";
const ARG_NAME_OUTPUT: &'static str = "output";
const ARG_NAME_INPUT: &'static str = "input";

fn main() {
    let matches = App::new("Caesar Cipher Tool")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Tool to (de)cipher text using the Caesar Cipher")
        .args(&[
            Arg::with_name(ARG_NAME_INPUT_TEXT)
                .help("Text for encoding/decoding")
                .required(true)
                .index(1),
            Arg::with_name(ARG_NAME_ROTATION)
                .help("Rotation value that should be used in the process")
                .required(true)
                .index(2),
            Arg::with_name(ARG_NAME_DECIPHER)
                .help("Decipher the input text")
                .short("d"),
            Arg::with_name(ARG_NAME_OUTPUT)
                .help("Write result to a file")
                .short("o")
                .takes_value(true),
            Arg::with_name(ARG_NAME_INPUT)
                .help("Read text from file")
                .short("i"),
        ])
        .get_matches();

    let text: String = if matches.is_present(ARG_NAME_INPUT) {
        io::read_input(matches.value_of(ARG_NAME_INPUT_TEXT).unwrap()).expect("Error reading file")
    } else {
        matches.value_of(ARG_NAME_INPUT_TEXT).unwrap().to_string()
    };

    let rotation = matches.value_of(ARG_NAME_ROTATION).unwrap();
    let rot = rotation.parse::<u8>().unwrap_or_default();

    let output = if matches.is_present(ARG_NAME_DECIPHER) {
        caesar::decipher(&text, rot)
    } else {
        caesar::cipher(&text, rot)
    };

    if matches.is_present(ARG_NAME_OUTPUT) {
        if let Some(file) = matches.value_of(ARG_NAME_OUTPUT) {
            io::write_output(file, &output).unwrap();
        }
    } else {
        println!("{}", output);
    }
}
