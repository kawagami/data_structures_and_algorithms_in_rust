use anyhow::{bail, Error, Ok, Result};
use base64::encode;
use hash::merhash::mersenne_hash;

const CRYPTO: &str = "plokmijnuhbygvtfcrdxeszwaqQAZWSXEDCRFVTGBYHUJMIKOLP";

pub fn generate_password(seed: &str, length: usize) -> Result<String, Error> {
    if length < 6 {
        bail!("長度需要大於 6")
    }

    let p = match length {
        6..=10 => 1,
        11..=15 => 2,
        16..=20 => 3,
        _ => 3,
    };
    let mut mer_hash = mersenne_hash(seed).pow(p);

    let mut passwd = String::new();
    let crypto_len = CRYPTO.len();

    while mer_hash > 9 {
        let loc = mer_hash & crypto_len;
        let nthc = CRYPTO.chars().nth(loc).expect("Error while getting char!");
        passwd.push(nthc);
        mer_hash /= crypto_len;
    }

    let interval = passwd.clone();
    for c in seed.chars() {
        passwd.push(c);
        passwd += &interval;
    }

    passwd = encode(passwd);
    passwd = passwd.replace("+", "*").replace("/", "*");

    let interval = passwd.clone();
    while passwd.len() < length {
        passwd += &interval;
    }

    Ok(format!("{}: {}", seed, &passwd[..length]))
}
