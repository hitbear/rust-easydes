extern crate clap;

use clap::{builder::BoolValueParser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use easydes::easydes::*;

fn main() {
    let arg_matches: ArgMatches = Command::new("easydes")
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
                .help("Encryption/Decryption key")
                .required(true),
        )
        .arg(
            Arg::new("iv")
                .short('i')
                .long("iv")
                .value_name("IV")
                .help("Encryption/Decryption key"),
        )
        .arg(
            Arg::new("if")
                .long("infile")
                .value_name("INPATH")
                .help("Specify the path to the input file."), //.required(false),
        )
        .arg(
            Arg::new("mode")
                .short('m')
                .value_name("MODE")
                .help("Specify the mode. Default is ECB which doesn't require an IV.")
                .possible_values(["ECB", "CBC"])
                .default_value("ECB"),
        )
        .arg(
            Arg::new("encrypt")
                .short('e')
                .help("Encrypt")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("decrypt")
                .short('d')
                .help("Decrypt")
                .action(ArgAction::SetFalse),
        )
        .group(
            ArgGroup::new("direction")
                .arg("encrypt")
                .arg("decrypt")
                .required(true),
        )
        .get_matches();

    let des_mode: Mode = match arg_matches.value_of("mode") {
        Some("ECB") => easydes::easydes::Mode::ECB,
        Some("CBC") => easydes::easydes::Mode::CBC,
        Some(_) => panic!("No such mode"),
        None => easydes::easydes::Mode::CBC,
    };

    let enc_or_dec: Des = match arg_matches.get_one::<bool>("direction") {
        Some(true) => Des::Encrypt,
        Some(false) => Des::Decrypt,
        None => panic!("It was not specified if encryption or decryption should be used."),
    };

    // ToDo: Only in cbc
    if des_mode == Mode::CBC && !arg_matches.is_present("iv") {
        panic!("We need an IV in CBC mode!");
    }

    let key_hex_string: &str = arg_matches.value_of("key").unwrap();
    let mut key: [u8; 8] = [0 as u8; 8];
    hex::decode_to_slice(key_hex_string, &mut key).expect("Decoding key failed");

    //let input: [u8; 9] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xaa];
    let input: [u8; 16] = [
    0x8f,
    0xd0,
    0xab,
    0x49,
    0xb2,
    0x8,
    0x2,
    0xd3,
    0x34,
    0x41,
    0xf3,
    0xd8,
    0xc9,
    0xf4,
    0xcf,
    0x2e,
];


    let output = match des_mode {
        Mode::ECB => des_ecb(&key, &mut input.to_vec(), enc_or_dec),
        Mode::CBC => {
            let mut iv: [u8; 8] = [0 as u8; 8];
            let iv_hex_string = arg_matches.value_of("key").unwrap();
            hex::decode_to_slice(iv_hex_string, &mut iv).expect("Decoding IV failed");
            des_cbc(&key, &iv, &mut input.to_vec(), enc_or_dec)
        }
    };

    println!("{:#02x?}", output);
}
