use hkdf::Hkdf;
use sha2::Sha256;
use ed25519_dalek::{Keypair, SecretKey, PublicKey, SIGNATURE_LENGTH};
use rand_core::OsRng;

/// Derive a deterministic, pairwise DID keypair from a master seed and site+jurisdiction context. [file:1]
pub fn derive_site_did_keypair(master_seed: &[u8; 32], site_origin: &str, jurisdiction_code: &str) -> Keypair {
    let info = format!("augmented-id:pairwise-did:{}:{}", site_origin, jurisdiction_code);
    let hk = Hkdf::<Sha256>::new(Some(master_seed), info.as_bytes());

    let mut okm = [0u8; 32];
    hk.expand(b"ed25519-pairwise", &mut okm).expect("HKDF expand");

    let secret = SecretKey::from_bytes(&okm).expect("ed25519 sk");
    let public = PublicKey::from(&secret);
    Keypair { secret, public }
}

/// Render a DID string with no stable global identifier exposed to any origin. [file:1]
pub fn derive_site_did(master_seed: &[u8; 32], site_origin: &str, jurisdiction_code: &str) -> String {
    let kp = derive_site_did_keypair(master_seed, site_origin, jurisdiction_code);
    let encoded_pk = base64::encode_config(kp.public.as_bytes(), base64::URL_SAFE_NO_PAD);
    format!("did:aug:{}/{}#{}", jurisdiction_code, hash_origin(site_origin), encoded_pk)
}

fn hash_origin(origin: &str) -> String {
    use sha2::Digest;
    let mut hasher = Sha256::new();
    hasher.update(origin.as_bytes());
    let hash = hasher.finalize();
    base64::encode_config(&hash[..16], base64::URL_SAFE_NO_PAD)
}
