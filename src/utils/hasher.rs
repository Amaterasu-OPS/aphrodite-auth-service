use sha2::{Digest, Sha512};

pub fn hash_sha512(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input);

    let digest = hasher.finalize();
    hex::encode(digest)
}

#[allow(dead_code)]
pub fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input);

    let digest = hasher.finalize();
    hex::encode(digest)
}