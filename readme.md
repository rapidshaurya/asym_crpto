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
## How to create openssl file for this project
- Private key generate
```
openssl genrsa -out key.pem 2048
openssl pkcs8 -topk8 -inform PEM -outform PEM -nocrypt -in key.pem -out private.pem
openssl asn1parse -in private.pem -out private_key.der
```
- pub key generate
```
openssl pkey -in private.pem -pubout -out pubkey.pem
openssl asn1parse -in pubkey.pem -out pub_key.der
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

