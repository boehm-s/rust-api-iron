/// WebSocket server to demonstrate ssl encryption within an a websocket server.
///
/// The resulting executable takes three arguments:
///   ADDR - The address to listen for incoming connections (e.g. 127.0.0:3012)
///   CERT - The path to the cert PEM (e.g. snakeoil.crt)
///   KEY - The path to the key PEM (e.g. snakeoil.key)
///
/// For more details concerning setting up the SSL context, see rust-openssl docs.

extern crate iron;
extern crate staticfile;
extern crate router;
extern crate mount;

extern crate ws;
extern crate clap;
#[cfg(feature="ssl")]
extern crate openssl;
extern crate env_logger;

#[cfg(feature="ssl")]
use std::rc::Rc;
#[cfg(feature="ssl")]
use openssl::ssl::{Ssl, SslContext, SslMethod};
#[cfg(feature="ssl")]
use openssl::x509::X509FileType;



use mount::Mount;
use router::Router;
use staticfile::Static;

use std::thread;
use std::path::Path;
#[cfg(feature="ssl")]
use std::result::{Result};

use iron::{Iron, Request, Response, IronResult, status};

#[cfg(not(feature="ssl"))]
use ws::listen;


#[cfg(feature="ssl")]
struct Server {
    out: ws::Sender,
    ssl: Rc<SslContext>,
}

#[cfg(feature="ssl")]
impl ws::Handler for Server {

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        self.out.send(msg) // simple echo
    }

    fn build_ssl(&mut self) -> ws::Result<Ssl> {
        Ssl::new(&self.ssl).map_err(ws::Error::from)
    }
}

fn health_test(req: &mut Request) -> IronResult<Response> {
    println!("Running health_test handler, URL path: {}", req.url.path().join("/"));
    Ok(Response::with((status::Ok, "This request was routed (health_test succeded) !")))
}


#[cfg(feature="ssl")]
fn main () {
    // Setup logging
    env_logger::init().unwrap();

    // setup command line arguments
    let matches = clap::App::new("WS-RS SSL Server Configuration")
        .version("1.0")
        .author("Jason Housley <housleyjk@gmail.com>")
        .about("Establish a WebSocket server that encrypts and decrypts messages.")
        .arg(clap::Arg::with_name("ADDR")
             .help("Address on which to bind the server.")
             .required(true)
             .index(1))
        .arg(clap::Arg::with_name("CERT")
             .help("Path to the SSL certificate.")
             .required(true)
             .index(2))
        .arg(clap::Arg::with_name("KEY")
             .help("Path to the SSL certificate key.")
             .required(true)
             .index(3))
        .get_matches();

    let mut context = SslContext::new(SslMethod::Tlsv1).unwrap();
    context.set_certificate_file(matches.value_of("CERT").unwrap(), X509FileType::PEM).unwrap();
    context.set_private_key_file(matches.value_of("KEY").unwrap(), X509FileType::PEM).unwrap();


    thread::spawn( move || {

        let context_rc = Rc::new(context);

        ws::Builder::new().with_settings(ws::Settings {
            encrypt_server: true,
            ..ws::Settings::default()
        }).build(|out: ws::Sender| {
            Server {
                out: out,
                ssl: context_rc.clone(),
            }
        }).unwrap().listen(matches.value_of("ADDR").unwrap()).unwrap();
    });


    let mut router = Router::new();
    router
        .get("/health_test", health_test, "health_test");

    let mut mount = Mount::new();
    mount
        .mount("/router", router)
        .mount("/", Static::new(Path::new("public")))
        .mount("/html", Static::new(Path::new("public/index.html")))
        .mount("/js/", Static::new(Path::new("public/js")));

    let key = Path::new("/etc/ssl/private/ssl-cert-snakeoil.key").to_path_buf();
    let cert = Path::new("/etc/ssl/certs/ssl-cert-snakeoil.pem").to_path_buf();


    match Iron::new(mount).https("127.0.0.1:3000", cert, key) {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    }


}

#[cfg(not(feature="ssl"))]
fn main() {
    let mut router = Router::new();
    router
        .get("/health_health_test", health_test, "health_test");

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/html", Static::new(Path::new("public/index.html")))
        .mount("/js/", Static::new(Path::new("public/js")));


    thread::spawn( move || {
        if let Err(error) = listen("127.0.0.1:3001", |out| {
            move |msg| {
                println!("Server got message '{}'. ", msg);
                out.send(msg)
            }
        }) {
            println!("Failed to create WebSocket due to {:?}", error);
        }
    });

    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
