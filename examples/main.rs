use chrono::Local;
use rsa::{pkcs8::DecodePrivateKey, PaddingScheme};
const RSA_2048_PRIV_DER: &[u8] = include_bytes!("../private_key.der");
const RSA_2048_PUB_DER: &[u8] = include_bytes!("../pub_key.der");
fn main() {
    let mut rng = rand::thread_rng();
    use rsa::{pkcs8::DecodePublicKey, PublicKey, RsaPrivateKey, RsaPublicKey};
    let pk = RsaPrivateKey::from_pkcs8_der(RSA_2048_PRIV_DER).unwrap();
    let a = Local::now().timestamp();
    let message = format!("{} shaurya@rapidinnovation.dev:123@Animal", a);
    let encb64 = base64::encode(message);
    println!("{}", encb64.clone());
    let data = encb64.as_bytes();
    let b = RsaPublicKey::from_public_key_der(RSA_2048_PUB_DER).unwrap();
    let mess = b
        .encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), data)
        .unwrap();
    println!("{:?}", mess);
    let dec_data = pk
        .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &mess)
        .expect("failed to decrypt");

    assert_ne!(&data[..], &mess[..]);

    let s = match std::str::from_utf8(&dec_data[..]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let decode = base64::decode(s).unwrap();

    let docode_str = String::from_utf8(decode).unwrap();

    println!("result: {}", docode_str);
}
