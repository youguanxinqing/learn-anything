use clap::{App, Arg};

fn main() {
    let matchs = App::new("echor")
        .version("0.1.0")
        .author("Mr guan")
        .about("Rust echo")
        .arg(
            // positional argument must appear at least once and can be repeated.
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(2),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();
    println!("{:#?}", matchs);
}
