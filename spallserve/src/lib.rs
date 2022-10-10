mod server;
mod server_options;

pub fn serve_project(raw_args: &Vec<String>) {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(serve_project_async(raw_args))
}

pub async fn serve_project_async(raw_args: &Vec<String>) {
    let options = parse_args(raw_args);
    server::serve(options).await;
}

fn parse_args(raw_args: &Vec<String>) -> server_options::ServerOptions {
    let mut options = server_options::ServerOptions {
        app_root: "".to_string(),
        port: 8000,
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
        let result = parser.parse(
            raw_args.clone(),
            &mut std::io::stdout(),
            &mut std::io::stderr(),
        );
        if let Err(error_code) = result {
            std::process::exit(error_code);
        }
    }
    options
}
