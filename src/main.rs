use actix_web::{dev::Service as _, middleware::Logger, App, HttpServer};
use clap::Parser;
use cmd_parser::CmdArguments;
use futures_util::future::FutureExt;
use log::{LevelFilter, SetLoggerError};
use logger::SimpleLogger;

mod cmd_parser;
mod logger;

static LOGGER: SimpleLogger = SimpleLogger;

fn init_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = CmdArguments::parse();
    init_logger().unwrap();
    if args.headers {
        HttpServer::new(|| {
            App::new()
                .wrap_fn(|req, srv| {
                    println!("-> {:?} \"{} {} {:?}\"", req.head().peer_addr.unwrap(), req.method(), req.head().uri, req.version());
                    println!("Headers:");
                    for (key, value) in req.headers() {
                        println!("  {}: {}", key, value.to_str().unwrap());
                    }
                    srv.call(req).map(|res| res)
                })
        })
        .bind((args.host, args.port))?
        .run()
        .await
    } else {
        HttpServer::new(|| App::new().wrap(Logger::default()))
            .bind((args.host, args.port))?
            .run()
            .await
    }
}
