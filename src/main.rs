use clap::{App, Arg};

mod caesar;

const ARG_NAME_INPUT_TEXT: &'static str = "text";
const ARG_NAME_ROTATION: &'static str = "rotation";

fn main() {
    let matches = App::new("Caesar Cipher Tool")
        .version("0.1")
        .author("José Duarte <jmg.duarte@campus.fct.unl.pt>")
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
        ])
        .get_matches();

    let text = matches.value_of(ARG_NAME_INPUT_TEXT).unwrap();
    let rotation = matches.value_of(ARG_NAME_ROTATION).unwrap();
    let rot = rotation.parse::<u8>().unwrap();

    println!("text:{}\n", text);
    println!("rotation:{}\n", rot);
    println!("result:{}\n", caesar::cipher(text, rot));
}
