use base64::Engine as _;
use rustls_pki_types::CertificateDer;
use sha2::Digest as _;

// com/hypixel/hytale/server/core/auth/CertificateUtil.java
pub fn compute_certificate_fingerprint(cert: &CertificateDer) -> String {
    let digest = sha2::Sha256::digest(cert);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest)
}
