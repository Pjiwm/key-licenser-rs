use std::hash::{DefaultHasher, Hash, Hasher};

use hmac::{Hmac, Mac};
use license_key::{Generator, HexFormat, KeyHasher};
use sha2::Sha512;

pub struct LicenseGenerator {
    generator: Generator<LicenseHasher>,
}
impl LicenseGenerator {
    pub fn new(license_secret: [u8; 32], iv: Vec<(u64, u64, u64)>) -> Self {
        Self {
            generator: Generator::new(LicenseHasher::new(license_secret), iv),
        }
    }

    pub fn create_license(&self, email: &str) -> String {
        let mut hasher = DefaultHasher::new();
        let seed = {
            email.hash(&mut hasher);
            hasher.finish()
        };
        let key = self.generator.generate(seed);
        key.serialize::<HexFormat>()
    }
}

type HmacSha512 = Hmac<Sha512>;

struct LicenseHasher {
    secret_key: [u8; 32],
}

impl LicenseHasher {
    pub fn new(secret_key: [u8; 32]) -> Self {
        LicenseHasher { secret_key }
    }
}

impl KeyHasher for LicenseHasher {
    fn hash(&self, seed: u64, a: u64, b: u64, c: u64) -> u64 {
        let mut mac =
            HmacSha512::new_from_slice(&self.secret_key).expect("HMAC can take key of any size");
        mac.update(&seed.to_le_bytes());
        mac.update(&a.to_le_bytes());
        mac.update(&b.to_le_bytes());
        mac.update(&c.to_le_bytes());

        let result = mac.finalize().into_bytes();
        u64::from_le_bytes(result[0..8].try_into().expect("HMAC output is too short"))
    }
}
