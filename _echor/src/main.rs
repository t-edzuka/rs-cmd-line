use clap::{App, Arg};

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Tomoya Edzuka")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .allow_invalid_utf8(true)
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print new line")
                .takes_value(false),
        )
        .get_matches();

    let text: Vec<String> = _matches
        .values_of_lossy("text")
        .unwrap();
    let omit_newline = _matches
        .is_present("omit_newline");
    // println!("{:#?}", text);

    let ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), ending);
}
