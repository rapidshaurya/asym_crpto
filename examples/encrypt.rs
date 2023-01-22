use chrono::Local;
use rsa::{PaddingScheme};
use rsa::{pkcs8::DecodePublicKey, PublicKey, RsaPublicKey};

const RSA_2048_PUB_DER: &[u8] = include_bytes!("../pub_key.der");
fn main() {
    let mut rng = rand::thread_rng();
    let timestamp = Local::now().timestamp();
    let message = format!("{} write message here", timestamp);
    let encb64 = base64::encode(message);
    let data = encb64.as_bytes();
    let rsa_pub_key = RsaPublicKey::from_public_key_der(RSA_2048_PUB_DER).unwrap();
    let mess = rsa_pub_key
        .encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), data)
        .unwrap();
    println!("{:?}", mess);

}
