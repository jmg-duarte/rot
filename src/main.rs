use clap::{App, Arg};

mod caesar;
mod io;

const ARG_NAME_INPUT_TEXT: &'static str = "text";
const ARG_NAME_ROTATION: &'static str = "rotation";
const ARG_NAME_DECIPHER: &'static str = "decipher";
const ARG_NAME_OUTPUT: &'static str = "output";
const ARG_NAME_INPUT: &'static str = "input";
const ARG_NAME_BLIND_BRUTE_FORCE: &'static str = "bbf";
const ARG_NAME_TEXT_EXPLICIT: &'static str = "explicit";

fn main() {
    let matches = App::new("Caesar Cipher Tool")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Tool to (de)cipher text using the Caesar Cipher")
        .args(&[
            Arg::with_name(ARG_NAME_ROTATION)
                .help("Rotation value that should be used in the process")
                //.required_unless(ARG_NAME_BLIND_BRUTE_FORCE)
                .index(1),
            Arg::with_name(ARG_NAME_INPUT_TEXT)
                .help("Text for encoding/decoding")
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
                .short("i")
                .requires(ARG_NAME_INPUT_TEXT),
            Arg::with_name(ARG_NAME_BLIND_BRUTE_FORCE)
                .help("Brute force all possible combinations")
                .takes_value(false)
                .long("blindf")
                .short("b")
                .conflicts_with_all(&[ARG_NAME_ROTATION, ARG_NAME_DECIPHER])
                .requires(ARG_NAME_TEXT_EXPLICIT),
            Arg::with_name(ARG_NAME_TEXT_EXPLICIT)
                .help("Explicitly pass some text")
                //.requires(ARG_NAME_BLIND_BRUTE_FORCE)
                .conflicts_with(ARG_NAME_INPUT_TEXT)
                .short("e")
                .takes_value(true)
                .number_of_values(1),
        ])
        .get_matches();

    let text: String = if matches.is_present(ARG_NAME_TEXT_EXPLICIT) {
        matches
            .value_of(ARG_NAME_TEXT_EXPLICIT)
            .unwrap()
            .to_string()
    } else if matches.is_present(ARG_NAME_INPUT_TEXT) {
        if matches.is_present(ARG_NAME_INPUT) {
            io::read_input(matches.value_of(ARG_NAME_INPUT_TEXT).unwrap())
                .expect("Error reading file")
        } else {
            matches.value_of(ARG_NAME_INPUT_TEXT).unwrap().to_string()
        }
    } else {
        io::read_stdin().unwrap()
    };

    let output = if matches.is_present(ARG_NAME_BLIND_BRUTE_FORCE) {
        caesar::brute_force(&text).join("\n")
    } else {
        let rotation = matches.value_of(ARG_NAME_ROTATION).unwrap();
        let rot = rotation.parse::<u8>().unwrap_or_default();

        if matches.is_present(ARG_NAME_DECIPHER) {
            caesar::decipher(&text, rot)
        } else {
            caesar::cipher(&text, rot)
        }
    };

    if matches.is_present(ARG_NAME_OUTPUT) {
        if let Some(file) = matches.value_of(ARG_NAME_OUTPUT) {
            io::write_output(file, &output).unwrap();
        }
    } else {
        println!("{}", output);
    }
}
