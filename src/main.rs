use crate::{
    digester::aes_digester::{self, AesCipher, AesMode},
    utils::{crc_util, hex_util},
};

pub mod core;
pub mod defi;
pub mod digester;
pub mod utils;

// just for testing
fn main() {
    let hex = "78FF0000CC020130044F50FD8BD08B11".trim();
    let r = crc_util::calculate_from_hex(defi::crc_enum::CrcType::Crc16Modbus, hex);
    match r {
        Err(e) => println!("Error: {}", e),
        Ok(crc) => println!("CRC: {}", crc),
    }
    let bts = hex_util::hex_to_bytes(hex).unwrap();
    let hex_str = hex_util::bytes_to_hex(&bts).unwrap();

    println!("converted hex : {}", hex_str);

    let number_hex = "004714CC";
    let num = hex_util::hex_to_f32_or_f64(number_hex).unwrap();

    println!("number : {}", num as f32);

    let n1: i16 = -22;
    let n1_ = hex_util::i16_to_hex(n1, 4).unwrap();
    println!("n1_ = {}", n1_);

    let h1 = "3322";
    let h1_ = hex_util::pad_hex_to_block_size(h1, 8, None).unwrap();
    println!("h1_ = {}", h1_);

    test_aes();
}

fn test_aes() {
    let key = b"0123456789abcdef";
    let data = b"Hello, AES ECBd!";

    println!("ori-data : {:?}", data);

    let cipher = AesCipher::new(key, AesMode::CBC).unwrap();

    let encrypted = cipher.encrypt(data, &[1]).unwrap();
    let decrypted = cipher.decrypt(&encrypted, &[1]).unwrap();

    println!("encrypted = {:?}", encrypted);
    println!("decrypted = {:?}", decrypted);

    // let encrypted_str = String::from_utf8(encrypted).unwrap();
    // let decrypted_str = String::from_utf8(decrypted).unwrap();

    // println!("encrypted_str = {}", encrypted_str);
    // println!("decrypted_str = {}", decrypted_str);
}
