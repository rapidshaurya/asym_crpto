
use rsa::{pkcs8::DecodePrivateKey, PaddingScheme};
use rsa::{RsaPrivateKey};
const RSA_2048_PRIV_DER: &[u8] = include_bytes!("../private_key.der");
fn main() {
   
    
    let rsa_private_key = RsaPrivateKey::from_pkcs8_der(RSA_2048_PRIV_DER).unwrap();
    let message:Vec<u8>=vec![]; // encrpted data 
    println!("decoded data: {}", decode(message, rsa_private_key));
}

fn decode(data:Vec<u8>, rsa_private_key: RsaPrivateKey)->String{

    let dec_data = rsa_private_key
        .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &data)
        .expect("failed to decrypt");


    let s = match std::str::from_utf8(&dec_data[..]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let decode = base64::decode(s).unwrap();

    let docode_str = String::from_utf8(decode).unwrap();

    return docode_str;

}