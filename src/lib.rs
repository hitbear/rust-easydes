mod constants;

pub mod easydes {

    pub const VERSION: &str = env!("CARGO_PKG_VERSION");

    pub enum Des {
        Encrypt, //0
        Decrypt, //1
    }

    #[derive(PartialEq)]
    pub enum Mode {
        ECB,
        CBC,
    }

    use crate::constants::constants::PERM_F;
    use crate::constants::constants::PERM_G;
    use crate::constants::constants::PERM_H;
    use crate::constants::constants::PERM_I;
    use crate::constants::constants::PERM_J;
    use crate::constants::constants::PERM_K;
    use crate::constants::constants::PERM_L;
    use crate::constants::constants::PERM_M;

    #[allow(dead_code)]
    fn add_padding(input: &mut Vec<u8>) {
        let len: usize = input.len();
        let rest_length: usize = 8 - len % 8;
        //let mut padding: Vec<u8> = vec![rest_length as u8; rest_length];
        let mut padding: Vec<u8> = vec![0x0 as u8; rest_length];
        input.append(&mut padding);
        // while len < new_len {
        //     input.push(padding_value as u8);
        //     len = len + 1;
        // }
    }

    #[allow(dead_code)]
    fn add_padding_16(input: &mut Vec<u8>) {
        let len: usize = input.len();
        let rest_length: usize = 16 - len % 16;
        //let mut padding: Vec<u8> = vec![rest_length as u8; rest_length];
        let mut padding: Vec<u8> = vec![0x0 as u8; rest_length];
        input.append(&mut padding);
    }

    #[allow(dead_code)]
    pub fn calculate_key_table(key: &[u8], out_table: &mut [[u8; 6]; 16]) {
        let mut tmp_56: [u8; 7] = [0 as u8; 7];

        for i in 0..7 {
            let byte: u8 = (0..8)
                .map(|j| {
                    let (div, rem) = ((PERM_H[i * 8 + j] - 1) / 8, (PERM_H[i * 8 + j] - 1) % 8);
                    let mut bit: u8 = (key[div as usize] >> (7 - rem)) & 0x01;
                    bit = bit << (7 - j);
                    bit
                })
                .sum();
            tmp_56[i] = byte;
        }

        for j in 0..16 {
            shift_e_array(&mut tmp_56, PERM_M[j]);

            for k in 0..6 {
                for l in 0..8 {
                    let (div, rem) = ((PERM_I[k * 8 + l] - 1) / 8, (PERM_I[k * 8 + l] - 1) % 8);
                    out_table[j][k] = out_table[j][k] << 1;
                    out_table[j][k] |= tmp_56[div as usize] >> (7 - rem) & 0x01;
                }
            }
        }
    }

    #[allow(dead_code)]
    fn shift_e_array(input: &mut [u8; 7], steps: u8) {
        let mut first_part: u32 =
            u32::from_be_bytes([input[0], input[1], input[2], input[3] & 0xF0]);
        let mut first_bit: u8 = 0;

        match steps {
            1 => {
                first_bit = input[0] & 0x80;
            }
            2 => {
                first_bit = input[0] & 0xC0;
            }
            _ => print!("Error"),
        }
        first_bit = first_bit >> (4 - steps);

        first_part = first_part << steps;
        first_part = first_part ^ first_bit as u32;

        let mut second_part: u32 =
            u32::from_be_bytes([input[3] & 0x0F, input[4], input[5], input[6]]);

        match steps {
            1 => {
                first_bit = input[3] & 0x08;
            }
            2 => {
                first_bit = input[3] & 0x0C;
            }
            _ => print!("Error"),
        }
        first_bit = first_bit >> (4 - steps);

        second_part = second_part << steps;
        second_part = second_part & 0x0FFFFFFF;
        second_part = second_part ^ first_bit as u32;

        input[0] = ((first_part & 0xFF000000) >> 24) as u8;
        input[1] = ((first_part & 0x00FF0000) >> 16) as u8;
        input[2] = ((first_part & 0x0000FF00) >> 8) as u8;

        input[3] = (first_part & 0x000000FF ^ ((second_part & 0xFF000000) >> 24)) as u8;

        input[4] = ((second_part & 0x00FF0000) >> 16) as u8;
        input[5] = ((second_part & 0x0000FF00) >> 8) as u8;
        input[6] = (second_part & 0x0000FF) as u8;
    }

    fn shift_and_divide(
        buffer: &mut [u8; 8],
        index: u8,
        direction: &Des,
        key_table: &[[u8; 6]; 16],
    ) {
        // let mut part1:Vec<u8> = buffer.clone().to_vec();
        // let part2: Vec<u8> = part1.split_off(buffer.len()/2);

        let mut flat_table: [u8; 6] = [0; 6];
        //let mut table_2: Vec<u8> = Vec::new();
        let mut table_2: [u8; 8] = [0; 8];

        // Only the first 48 (of 64) bits are calculated.

        for i in 0..6 {
            let byte: u8 = (0..8)
                .map(|j| {
                    let (div, rem) = ((PERM_J[i * 8 + j] - 1) / 8, (PERM_J[i * 8 + j] - 1) % 8);
                    //let mut bit = (part2[div as usize] >> (7 - rem)) & 0x01;
                    let mut bit: u8 = (buffer[div as usize + 4] >> (7 - rem)) & 0x01;
                    bit = bit << (7 - j);
                    bit
                })
                .sum();

            flat_table[i] = byte ^ key_table[index as usize][i];
        }

        let mut pos: usize = 0;
        for i in (0..4).step_by(3) {
            let mut flat: u16 = u16::from_be_bytes([flat_table[i], flat_table[i + 1]]);
            //table_2.push(((flat & 0xFC00) >> 10)  as u8);
            table_2[pos] = ((flat & 0xFC00) >> 10) as u8;
            pos = pos + 1;
            //table_2.push(((flat & 0x03F0) >> 4)  as u8);
            table_2[pos] = ((flat & 0x03F0) >> 4) as u8;
            pos = pos + 1;

            flat = u16::from_be_bytes([flat_table[i + 1], flat_table[i + 2]]);
            // table_2.push(((flat & 0x0FC0) >> 6)  as u8);
            // table_2.push((flat & 0x003F)  as u8);
            table_2[pos] = ((flat & 0x0FC0) >> 6) as u8;
            pos = pos + 1;

            table_2[pos] = (flat & 0x003F) as u8;
            pos = pos + 1;
        }

        // key length is 32 bit (4 byte)
        //let mut key: Vec<u8> = Vec::new();
        let mut key: [u8; 4] = [0; 4];

        let mut pre_key: u8;

        for i in (0..8).step_by(2) {
            let mut val = table_2[i];
            let mut index_1 = ((val & 0x20) >> 4) | (val & 0x01);
            let mut index_2 = (val & 0x1e) >> 1;

            pre_key = PERM_L[i][index_1 as usize][index_2 as usize];

            val = table_2[i + 1];
            index_1 = ((val & 0x20) >> 4) | (val & 0x01);
            index_2 = (val & 0x1e) >> 1;

            pre_key = pre_key << 4;
            pre_key |= PERM_L[i + 1][index_1 as usize][index_2 as usize];

            key[i / 2] = pre_key;
        }

        match (direction, index) {
            (Des::Decrypt, 0) | (Des::Encrypt, 15) => {
                for i in 0..4 {
                    let mut byte: u8 = 0;
                    for j in 0..8 {
                        let (div, rem) = ((PERM_K[i * 8 + j] - 1) / 8, (PERM_K[i * 8 + j] - 1) % 8);
                        byte = byte << 1;
                        byte |= (key[div as usize] >> (7 - rem)) & 0x01;
                    }

                    // let byte: u8 = (0..8).map(|j|{
                    //     let (div, rem) = ((PERM_K[i * 8 + j] - 1) / 8, (PERM_K[i * 8 + j] - 1) % 8);
                    //     let mut bit = (key[div as usize] >> (7 - rem)) & 0x01;
                    //     bit = bit << (7 - j);
                    //     bit
                    // }).sum();

                    buffer[i] = buffer[i] ^ byte;
                }
            }
            (_, _) => {
                for i in 0..4 {
                    let mut byte: u8 = 0;
                    for j in 0..8 {
                        let (div, rem) = ((PERM_K[i * 8 + j] - 1) / 8, (PERM_K[i * 8 + j] - 1) % 8);
                        byte = byte << 1;
                        byte |= (key[div as usize] >> (7 - rem)) & 0x01;
                    }

                    // let byte: u8 = (0..8).map(|j| {
                    //     let (div, rem) = ((PERM_K[i * 8 + j] - 1) / 8, (PERM_K[i * 8 + j] - 1) % 8);
                    //     let mut bit = (key[div as usize] >> (7 - rem)) & 0x01;
                    //     bit = bit << (7 - j);
                    //     bit
                    // }).sum();

                    let tmp: u8 = buffer[i + 4];
                    buffer[i + 4] = buffer[i] ^ byte;
                    buffer[i] = tmp;
                }
            }
        }
    }

    fn encrypt_frame(input: &[u8], direction: &Des, key_table: &[[u8; 6]; 16]) -> [u8; 8] {
        let mut tmp: [u8; 8] = [0; 8];

        // flip bits
        for i in 0..input.len() {
            // let mut byte: u8 = 0;
            // for j in 0..8 {
            //     let (div, rem) = ((PERM_F[i*8 + j] - 1) / 8, (PERM_F[i*8 + j] - 1) % 8);
            //     byte = byte << 1;
            //     byte |= input[div as usize] >> (7 - rem) & 0x01;
            // }

            let byte: u8 = (0..8)
                .map(|j| {
                    let (div, rem) = ((PERM_F[i * 8 + j] - 1) / 8, (PERM_F[i * 8 + j] - 1) % 8);
                    let mut bit = input[div as usize] >> (7 - rem) & 0x01;
                    bit = bit << (7 - j);
                    bit
                })
                .sum();

            tmp[i] = byte;
        }

        match direction {
            // decrypt
            Des::Decrypt => {
                for i in (0..16).rev() {
                    shift_and_divide(&mut tmp, i, &direction, key_table);
                }
            }
            // encrypt
            Des::Encrypt => {
                for i in 0..16 {
                    shift_and_divide(&mut tmp, i, &direction, key_table);
                }
            } // _ => {
              //     println!("Error, no such mode");
              // }
        }

        let mut out: [u8; 8] = [0; 8];
        for i in 0..8 {
            let byte: u8 = (0..8)
                .map(|j| {
                    let (div, rem) = ((PERM_G[i * 8 + j] - 1) / 8, (PERM_G[i * 8 + j] - 1) % 8);
                    let mut bit = tmp[div as usize] >> (7 - rem) & 0x01;
                    bit = bit << (7 - j);
                    bit
                })
                .sum();

            out[i] = byte;
        }

        // The following code seems to be slower - because we need Vec?

        // let out: Vec<u8> = (0..8).map(|i|{
        //     let byte: u8 = (0..8).map(|j|{
        //         let (div, rem) = ((PERM_G[i*8 + j] - 1) / 8, (PERM_G[i*8 + j] - 1) % 8);
        //         let mut bit = tmp[div as usize] >> (7 - rem) & 0x01;
        //         bit = bit << (7 - j);
        //         bit
        //         }).sum();
        //         byte
        // }).collect();

        out
    }

    pub fn des_ecb(key: &[u8], input: &mut Vec<u8>, direction: Des) -> Vec<u8> {
        if input.len() % 8 != 0 {
            add_padding(input);
        }

        let num_frames: usize = input.len() / 8;

        let mut key_table: [[u8; 6]; 16] = [[0; 6]; 16];
        calculate_key_table(key, &mut key_table);

        let mut output: Vec<u8> = Vec::new();
        for frame_ctr in 0..num_frames {
            let frame: &mut [u8] = &mut input[(frame_ctr * 8)..((frame_ctr + 1) * 8)];
            let block: [u8; 8] = encrypt_frame(&frame, &direction, &key_table);

            output.extend_from_slice(&block);
        }

        output
    }

    pub fn des_cbc(key: &[u8], iv: &[u8], input: &mut Vec<u8>, direction: Des) -> Vec<u8> {
        if input.len() % 8 != 0 {
            add_padding(input);
        }

        let num_frames: usize = input.len() / 8;

        let mut key_table: [[u8; 6]; 16] = [[0; 6]; 16];
        calculate_key_table(key, &mut key_table);

        let mut output: Vec<u8> = Vec::new();
        match direction {
            Des::Encrypt => {
                for frame_ctr in 0..num_frames {
                    let frame: &mut [u8] = &mut input[(frame_ctr * 8)..((frame_ctr + 1) * 8)];

                    match frame_ctr {
                        0 => {
                            for i in 0..8 {
                                frame[i] = frame[i] ^ iv[i];
                            }
                        }
                        _ => {
                            for i in 0..8 {
                                frame[i] = frame[i] ^ output[((frame_ctr - 1) * 8) + i];
                            }
                        }
                    }

                    let block: [u8; 8] = encrypt_frame(&frame, &direction, &key_table);

                    output.extend_from_slice(&block);
                }
            }
            Des::Decrypt => {
                for frame_ctr in 0..num_frames {
                    let frame: &mut [u8] = &mut input[(frame_ctr * 8)..((frame_ctr + 1) * 8)];
                    let mut block: [u8; 8] = encrypt_frame(&frame, &direction, &key_table);

                    match frame_ctr {
                        0 => {
                            for i in 0..8 {
                                block[i] = block[i] ^ iv[i];
                            }
                        }
                        _ => {
                            for i in 0..8 {
                                block[i] = block[i] ^ input[((frame_ctr - 1) * 8) + i];
                            }
                        }
                    }

                    output.extend_from_slice(&block);
                }
            }
        }

        output
    }



    pub fn triple_des_ecb(key: &[u8], input: &mut Vec<u8>, direction: Des) -> Vec<u8> {
        let mut key1: Vec<u8> = key.to_vec();
        let key3: Vec<u8> = key1.split_off(16);
        let key2: Vec<u8> = key1.split_off(8);

        match direction {
            Des::Encrypt => {
                let mut tmp: Vec<u8> = des_ecb(key1.as_slice(), input, Des::Encrypt);
                let mut tmp2: Vec<u8> = des_ecb(key2.as_slice(), &mut tmp, Des::Decrypt);
                let output: Vec<u8> = des_ecb(key3.as_slice(), &mut tmp2, Des::Encrypt);
                output
            }
            Des::Decrypt => {
                let mut tmp: Vec<u8> = des_ecb(key3.as_slice(), input, Des::Decrypt);
                let mut tmp2: Vec<u8> = des_ecb(key2.as_slice(), &mut tmp, Des::Encrypt);
                let output: Vec<u8> = des_ecb(key1.as_slice(), &mut tmp2, Des::Decrypt);
                output
            }
        }
    }


    pub fn triple_des_cbc(key: &[u8],iv: &[u8], input: &mut Vec<u8>, direction: Des) -> Vec<u8> {
        let mut key1: Vec<u8> = key.to_vec();
        let key3: Vec<u8> = key1.split_off(16);
        let key2: Vec<u8> = key1.split_off(8);

        if input.len() % 8 != 0 {
            add_padding(input);
        }

        let num_frames: usize = input.len() / 8;

        let mut key_table1: [[u8; 6]; 16] = [[0; 6]; 16];
        calculate_key_table(key, &mut key_table1);

        let mut key_table2: [[u8; 6]; 16] = [[0; 6]; 16];
        calculate_key_table(key, &mut key_table2);

        let mut key_table3: [[u8; 6]; 16] = [[0; 6]; 16];
        calculate_key_table(key, &mut key_table3);

        let mut output: Vec<u8> = Vec::new();
        match direction {
            Des::Encrypt => {
                for frame_ctr in 0..num_frames {
                    let frame: &mut [u8] = &mut input[(frame_ctr * 8)..((frame_ctr + 1) * 8)];

                    match frame_ctr {
                        0 => {
                            for i in 0..8 {
                                frame[i] = frame[i] ^ iv[i];
                            }
                        }
                        _ => {
                            for i in 0..8 {
                                frame[i] = frame[i] ^ output[((frame_ctr - 1) * 8) + i];
                            }
                        }
                    }

                    let block1: [u8; 8] = encrypt_frame(&frame, &Des::Encrypt, &key_table1);
                    let block2: [u8; 8] = encrypt_frame(&block1, &Des::Decrypt, &key_table2);
                    let block3: [u8; 8] = encrypt_frame(&block2, &Des::Encrypt, &key_table3);

                    output.extend_from_slice(&block3);
                }
            }
            Des::Decrypt => {
                for frame_ctr in 0..num_frames {
                    let frame: &mut [u8] = &mut input[(frame_ctr * 8)..((frame_ctr + 1) * 8)];
                    let block1: [u8; 8] = encrypt_frame(&frame, &Des::Decrypt, &key_table1);
                    let block2: [u8; 8] = encrypt_frame(&block1, &Des::Encrypt, &key_table2);
                    let mut block3: [u8; 8] = encrypt_frame(&block2, &Des::Decrypt, &key_table3);

                    match frame_ctr {
                        0 => {
                            for i in 0..8 {
                                block3[i] = block3[i] ^ iv[i];
                            }
                        }
                        _ => {
                            for i in 0..8 {
                                block3[i] = block3[i] ^ input[((frame_ctr - 1) * 8) + i];
                            }
                        }
                    }

                    output.extend_from_slice(&block3);
                }
            }
        }

        output
    }
}

mod test_constants;
#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_constants::test_constants::CONTROL_KEY_TABLE;

    #[test]
    fn generate_key() {
        let key: [u8; 8] = [0x13, 0x34, 0x57, 0x79, 0x9B, 0xBC, 0xDF, 0xF1];
        let mut key_table: [[u8; 6]; 16] = [[0; 6]; 16];

        easydes::calculate_key_table(&key, &mut key_table);

        println!("{:#02x?}", key_table);
        assert_eq!(key_table, CONTROL_KEY_TABLE);
    }

    #[test]
    fn encrypt_ecb() {
        let plaintext: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        let key: [u8; 8] = [0x13, 0x34, 0x57, 0x79, 0x9B, 0xBC, 0xDF, 0xF1];
        let mut ciphertext: Vec<u8> =
            easydes::des_ecb(&key, &mut plaintext.to_vec(), easydes::Des::Encrypt);

        let plaintext_again = easydes::des_ecb(&key, &mut ciphertext, easydes::Des::Decrypt);

        println!("{:#02x?}", plaintext_again);
        assert_eq!(plaintext.to_vec(), plaintext_again);
    }

    #[test]
    fn encrypt_cbc_empty_iv() {
        let plaintext: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        let key: [u8; 8] = [0x13, 0x34, 0x57, 0x79, 0x9B, 0xBC, 0xDF, 0xF1];
        let iv: [u8; 8] = [0x00 as u8; 8];
        let mut ciphertext: Vec<u8> =
            easydes::des_cbc(&key, &iv, &mut plaintext.to_vec(), easydes::Des::Encrypt);

        // with an empty iv, there should be no difference between ECB and CBC.
        let plaintext_again = easydes::des_ecb(&key, &mut ciphertext, easydes::Des::Decrypt);

        println!("{:#02x?}", plaintext_again);
        assert_eq!(plaintext.to_vec(), plaintext_again);
    }

    #[test]
    fn encrypt_cbc() {
        let plaintext: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        let key: [u8; 8] = [0x13, 0x34, 0x57, 0x79, 0x9B, 0xBC, 0xDF, 0xF1];
        let iv: [u8; 8] = [0x01 as u8; 8];
        let mut ciphertext: Vec<u8> =
            easydes::des_cbc(&key, &iv, &mut plaintext.to_vec(), easydes::Des::Encrypt);

        println!("{:#02x?}", ciphertext);

        let plaintext_again: Vec<u8> =
            easydes::des_cbc(&key, &iv, &mut ciphertext, easydes::Des::Decrypt);

        println!("{:#02x?}", plaintext_again);
        assert_eq!(plaintext.to_vec(), plaintext_again);
    }

    #[test]
    fn cbc_strings() {
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

        while plaintext_again.last() == Some(&0x00) {
            plaintext_again.pop();
        }

        assert_eq!(
            String::from_utf8(plaintext.as_bytes().to_vec()),
            String::from_utf8(plaintext_again)
        );
    }

    #[test]
    fn triple_des_ecb_strings() {
        let plaintext: &str = "HelloWorldHelloWorld";
        let key: [u8; 24] = [
            0x13, 0x34, 0x57, 0x79, 0x9B, 0xBC, 0xDF, 0xF1, 0x13, 0x34, 0x57, 0x79, 0x9B, 0xBC,
            0xDF, 0xF1, 0x13, 0x34, 0x57, 0x79, 0x9B, 0xBC, 0xDF, 0xF1,
        ];

        let mut ciphertext: Vec<u8> = easydes::triple_des_ecb(
            &key,
            &mut plaintext.as_bytes().to_vec(),
            easydes::Des::Encrypt,
        );

        println!("{:#02x?}", ciphertext);

        let mut plaintext_again: Vec<u8> =
            easydes::triple_des_ecb(&key, &mut ciphertext, easydes::Des::Decrypt);

        while plaintext_again.last() == Some(&0x00) {
            plaintext_again.pop();
        }

        println!("{:?}", String::from_utf8(plaintext_again.clone()).unwrap());

        assert_eq!(
            String::from_utf8(plaintext.as_bytes().to_vec()).unwrap(),
            String::from_utf8(plaintext_again).unwrap()
        );
    }
}
