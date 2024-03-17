use clap::{Arg, ArgMatches, Command};

use easydes::easydes::VERSION;

fn main() {
    let matches = Command::new("easydes")
        .version(VERSION)
        .about("Encrypt and decrypt with DES.")
        .arg(
            Arg::new("v")
                .short('v')
                //.multiple_occurrences(true)
                .help("Enable verbose logging"),
        )
        .arg(
            Arg::new("key")
            .short('k')
            .long("key")
            .value_name("KEY")
            .help(
                "Encryption/Decryption key"
            ),
        )
        .arg(
            Arg::new("iv")
            .short('i')
            .long("iv")
            .value_name("IV")
            .help(
                "Encryption/Decryption key"
            ),
        )
        .get_matches();
}
