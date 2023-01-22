# Aysm Crpto
## This project will help you to encrypt and decrypt data using asymmetic crptography

## Run Code
- For encrypt data
```
cargo run --example encrypt
```
- For decrypt data
```
cargo run --example encrypt
```
#### Note
- Please add pub_key.der and private_key.der outside the src folder
- ⚠️ Please don't change cargo.toml dependencies
- Use RSA-2048 PKCS#8 private key encoded as ASN.1 DER
- Use RSA-2048 `SubjectPublicKeyInfo` encoded as ASN.1 DER
#### References 
- [RSA crate](https://crates.io/crates/rsa)
- [base64 crate](https://crates.io/crates/base64)
- [Openssl commands](https://www.openssl.org/docs/man3.0/man1/)

