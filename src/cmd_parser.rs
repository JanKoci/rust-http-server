use clap::Parser;

const APP_DESCRIPTION: &str = "Simple Http server that logs all requests to stdout";
const APP_NAME: &str = "http-logger";
const APP_AUTHOR: &str = "Jan Koci";

#[derive(Parser, Debug)]
#[clap(
about = APP_DESCRIPTION,
name = APP_NAME,
author = APP_AUTHOR
)]
pub struct CmdArguments {
    #[clap(
        short = 'p',
        long = "port",
        default_value = "7878",
        help = "Port to listen on"
    )]
    pub port: u16,

    #[clap(
        short = 'a',
        long = "address",
        default_value = "127.0.0.1",
        help = "Host address to listen on"
    )]
    pub host: String,

    #[clap(
        short = 'H',
        long = "headers",
        default_value = "false",
        help = "Prints http headers"
    )]
    pub headers: bool,
}
