# Easydes

## Encrypt with DES easyly in Rust.

This is a Rust library that implements the DES cryptographic algorithm. 
##Running 

    easydes --help
    easydes 0.1.0
    Encrypt and decrypt with DES.

    USAGE:
        easydes [OPTIONS] --key <KEY> --infile <INPATH> <-e|-d>
    
    OPTIONS:
        -d                         Decrypt
        -e                         Encrypt
        -h, --help                 Print help information
        -i, --iv <IV>              IV
            --infile <INPATH>      Specify the path to the input file.
        -k, --key <KEY>            Encryption/Decryption key
        -m <MODE>                  Specify the mode. Default is ECB which doesn't require an IV.
                                   [default: ECB] [possible values: ECB, CBC]
            --outfile <OUTPATH>    Specify the path to the output file.
        -v                         Enable verbose logging
        -V, --version              Print version information

### Example
To encrypt a file, run

    easydes --key 133457799BBCDFF1 --iv 0000000000000000 -m CBC -e --infile tests/infile.txt  --outfile output.enc

To decrypt this file, you can run

    easydes --key 133457799BBCDFF1 --iv 0000000000000000 -m CBC -d --infile output.enc  --outfile plaintext.txt

### Build

To build the binary with [cargo](https://doc.rust-lang.org/cargo/), run

    cargo build --release

### Using the library

By including the crate with 

    use easydes::easydes::*;

And then


    let plaintext: &str = "HelloWorldHelloWorld";
    let key: [u8; 8] = [0x13, 0x34, 0x57, 0x79, 0x9B, 0xBC, 0xDF, 0xF1];
    let iv: [u8; 8] = [0x01 as u8; 8];

    let mut ciphertext = easydes::des_cbc(
        &key,
        &iv,
        &mut plaintext.as_bytes().to_vec(),
        easydes::Des::Encrypt,
    );

    println!("{:#02x?}", ciphertext);

    let mut plaintext_again: Vec<u8> =
        easydes::des_cbc(&key, &iv, &mut ciphertext, easydes::Des::Decrypt);

### Tests
Please see [TESTING.md](TESTING.md).

### License 
Please read [LICENSE.md](LICENSE.md).