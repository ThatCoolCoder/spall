mod server;
mod server_options;

#[tokio::main]
async fn main() {
    let options = parse_args();
    server::serve(options).await;
}

fn parse_args() -> server_options::ServerOptions {
    let mut options = server_options::ServerOptions {
        app_root: "".to_string(),
        port: 80,
    };
    // Set up argparser and use it
    {
        let mut parser = argparse::ArgumentParser::new();
        parser.refer(&mut options.port).add_option(
            &["-p", "--port"],
            argparse::Store,
            "Port to run the server on",
        );
        parser.refer(&mut options.app_root).add_argument(
            "path",
            argparse::Store,
            "Path to root of application",
        );
        parser.parse_args_or_exit();
    }
    options
}
