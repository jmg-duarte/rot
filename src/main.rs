use clap::values_t;
use clap::App;

mod caesar;
mod io;

fn main() {
    let yml = clap::load_yaml!("cli.yml");
    let matches = App::from_yaml(yml)
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .get_matches();

    let text = get_text(&matches);
    let output = process_text(&matches, &text);

    if matches.is_present("output") {
        let file = matches.value_of("output").unwrap();
        io::write_output(file, &output).expect("Error writing file");
    } else {
        println!("{}", output);
    };
}

fn get_text(matches: &clap::ArgMatches) -> String {
    let (input, file, stdin) = (
        matches.is_present("input"),
        matches.is_present("file"),
        matches.is_present("stdin"),
    );

    match (input, file, stdin) {
        (true, _, _) => {
            io::read_input(matches.value_of("input").unwrap()).expect("Error reading file")
        }
        (_, true, _) => matches.value_of("input").unwrap().to_string(),
        (_, _, true) => io::read_stdin().unwrap(),
        (_, _, _) => unreachable!(),
    }
}

fn process_text(matches: &clap::ArgMatches, text: &str) -> String {
    let (bforce, cipher, decipher) = (
        matches.is_present("brutef"),
        matches.is_present("cipher"),
        matches.is_present("decipher"),
    );

    match (bforce, cipher, decipher) {
        (true, _, _) => {
            let r: Vec<u8> = (1..26).collect();
            caesar::decipher_n(&text, &r)
        }
        (_, true, _) => {
            let v = clap::values_t!(matches, "decipher", u8).unwrap();
            caesar::decipher_n(&text, &v)
        }
        (_, _, true) => {
            let v = clap::values_t!(matches, "cipher", u8).unwrap();
            caesar::cipher_n(&text, &v)
        }
        (_, _, _) => unreachable!(),
    }
    .join("\n")
}
