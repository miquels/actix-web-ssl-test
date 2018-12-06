use std::io::Read;

use actix_web::{server, App, HttpRequest, Responder};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use native_tls;

fn index(_req: &HttpRequest) -> impl Responder {
    ""
}

fn ssl_builder() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .expect("failed to open/read key.pem");
    builder.set_certificate_chain_file("cert.pem")
        .expect("failed to open/read cert.pem");
    builder
}

pub fn tls_acceptor() -> native_tls::TlsAcceptor {
    let mut file = std::fs::File::open("cert+key.p12")
        .map_err(|e| {
            println!("failed to open .p12");
            println!("try running: openssl pkcs12 -export -passout pass:'' -out cert+key.p12 -inkey key.pem -in cert.pem");
            e
        }).expect("opening .p12");
    let mut der = vec![];
    file.read_to_end(&mut der).unwrap();
    let cert = native_tls::Identity::from_pkcs12(&der, "").expect("failed to read .p12");
    let tls_cx = native_tls::TlsAcceptor::builder(cert).build().unwrap();
    native_tls::TlsAcceptor::from(tls_cx)
}

fn main() {
    // load ssl keys
    let ssl_builder = ssl_builder();
    let tls_acceptor = tls_acceptor();

    let server = server::new(|| App::new().resource("/{any:.*}", |r| r.f(index)))
        .bind("127.0.0.1:8080").expect("bind 8080")
        .bind_ssl("127.0.0.1:8081", ssl_builder).expect("bind 8081")
        .bind_tls("127.0.0.1:8082", tls_acceptor).expect("bind 8082");
    println!("listening on localhost port 8080 (text) 8081 (openssl) 8082 (native_tls)");
    server.run();
}
