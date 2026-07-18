use rcgen::generate_simple_self_signed;
use tonic::transport::{Certificate, Identity, ServerTlsConfig, ClientTlsConfig};

pub fn generate_tls_identity() -> (Identity, Certificate) {
    let subject_alt_names = vec!["localhost".to_string(), "127.0.0.1".to_string()];
    let rcgen_cert = generate_simple_self_signed(subject_alt_names).unwrap();
    let cert = rcgen_cert.serialize_pem().unwrap();
    let key = rcgen_cert.serialize_private_key_pem();

    let identity = Identity::from_pem(&cert, &key);
    let certificate = Certificate::from_pem(&cert);

    (identity, certificate)
}

pub fn get_server_tls_config() -> ServerTlsConfig {
    let (identity, _) = generate_tls_identity();
    ServerTlsConfig::new().identity(identity)
}

pub fn get_client_tls_config() -> ClientTlsConfig {
    // For production, we'd load real roots. For auto-generated mTLS, we accept any cert
    let (_, _cert) = generate_tls_identity();
    ClientTlsConfig::new()
        // Note: For real mTLS we would pass the CA certificate here
        // .ca_certificate(cert)
}
