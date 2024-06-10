use std::{
    borrow::BorrowMut,
    fs,
    io::Read,
    path::{self, Path},
};

use anyhow::Ok;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{ed25519::signature::SignerMut, Signature, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{
    cli::text::{
        TextKeyGenerateOpts, TextSignFormat, TextSignOpts, TextSubcommand, TextVerifyOpts,
    },
    utils::{password, reader::get_reader},
};

pub fn process_text(cmd: TextSubcommand) -> anyhow::Result<()> {
    match cmd {
        TextSubcommand::Sign(TextSignOpts { input, key, format }) => {
            process_sign(&input, &key, format)
        }
        TextSubcommand::Verify(TextVerifyOpts {
            input,
            key,
            sig,
            format,
        }) => process_verify(&input, &key, sig.trim_end().as_bytes(), format),
        TextSubcommand::Generate(TextKeyGenerateOpts { output, format }) => {
            process_generate(format, output)
        }
    }
}

trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, reader: impl Read, sig: &[u8]) -> anyhow::Result<bool>;
}

trait TextLoader {
    fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self>
    where
        Self: Sized;
}

trait TextKeyGenerator {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>;
}

struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf: Vec<_> = Vec::new();
        reader.read_to_end(&mut buf)?;

        buf.extend_from_slice(&self.key);
        Ok(blake3::hash(&buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf: Vec<_> = Vec::new();
        reader.read_to_end(&mut buf)?;

        buf.extend_from_slice(&self.key);
        let hash = blake3::hash(&buf).as_bytes().to_vec();
        Ok(hash == sig)
    }
}

impl TextKeyGenerator for Blake3 {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let key = password::process_genpass(32, true, true, true, true)?;

        let mut keys: Vec<_> = Vec::new();
        keys.push(key.as_bytes().to_vec());

        Ok(keys)
    }
}

impl TextLoader for Blake3 {
    fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        let key = key.try_into().unwrap();
        Ok(Blake3::new(key))
    }
}

struct Ed25519Signer {
    key: SigningKey,
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf: Vec<_> = Vec::new();
        reader.read_to_end(&mut buf)?;

        let buf = buf.as_slice();
        Ok(self.key.clone().borrow_mut().sign(&buf).to_vec())
    }
}

impl TextKeyGenerator for Ed25519Signer {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;

        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let mut keys = Vec::new();
        keys.push(signing_key.as_bytes().to_vec());
        keys.push(verifying_key.as_bytes().to_vec());
        Ok(keys)
    }
}

impl TextLoader for Ed25519Signer {
    fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        let key = key.try_into().unwrap();
        let signing_key = SigningKey::from_bytes(&key);
        Ok(Ed25519Signer { key: signing_key })
    }
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf: Vec<_> = Vec::new();
        reader.read_to_end(&mut buf)?;

        let sig = Signature::from_bytes(sig.try_into().unwrap());
        let result = self.key.verify(buf.as_slice(), &sig);
        Ok(result.is_ok())
    }
}

impl TextLoader for Ed25519Verifier {
    fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        let key = key.try_into().unwrap();
        let verifying_key = VerifyingKey::from_bytes(&key)?;
        Ok(Ed25519Verifier { key: verifying_key })
    }
}

fn process_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => Blake3::load(key)?.sign(&mut reader)?,
        TextSignFormat::Ed25519 => Ed25519Signer::load(key)?.sign(&mut reader)?,
    };

    let signed = URL_SAFE_NO_PAD.encode(&signed);
    println!("{}, len is {}", signed, signed.len());

    Ok(())
}

fn process_verify(
    input: &str,
    key: &str,
    sig: &[u8],
    format: TextSignFormat,
) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;

    let sig = URL_SAFE_NO_PAD.decode(sig)?;

    let verifier = match format {
        TextSignFormat::Blake3 => Blake3::load(key)?.verify(&mut reader, &sig)?,
        TextSignFormat::Ed25519 => Ed25519Verifier::load(key)?.verify(&mut reader, &sig)?,
    };

    println!("verified result is: {}", verifier);

    Ok(())
}

fn process_generate(format: TextSignFormat, output: path::PathBuf) -> anyhow::Result<()> {
    match format {
        TextSignFormat::Blake3 => {
            let keys = Blake3::generate()?;
            let target = output.join("blake3.txt");
            fs::write(target, String::from_utf8(keys[0].clone())?)?
        }
        TextSignFormat::Ed25519 => {
            let keys = Ed25519Signer::generate()?;
            let sk_target = output.join("sk");
            fs::write(sk_target, &keys[0])?;

            let pk_target = output.join("pk");
            fs::write(pk_target, &keys[1])?;
        }
    }

    Ok(())
}
