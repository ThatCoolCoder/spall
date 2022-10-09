use std::convert::Infallible;
use std::fs;
use std::net::SocketAddr;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use hyper::service::{make_service_fn, service_fn};
use hyper::{http, Body, Request, Response, Server};

use crate::server_options::ServerOptions;

// This is my first code using async, lifetimes and all of that stuff so apologies if it's a bit weird/bad

const SPA_FILE_NAME: &'static str = "index.html";
const STATIC_DIR_NAME: &'static str = "static";
const SCRIPT_DIR_NAME: &'static str = "scripts";

enum RequestedItem<'a> {
    SPA,
    StaticFile(&'a Path),
    Invalid,
}

async fn serve_route(
    req: Request<Body>,
    app_root: Arc<PathBuf>,
) -> Result<Response<Body>, Infallible> {
    let path = Path::new(req.uri().path());
    let requested_item = determine_requested_item(&path);

    let response = match requested_item {
        RequestedItem::SPA => serve_spa(&*app_root),
        RequestedItem::StaticFile(file_path) => serve_static_file(file_path, &*app_root),
        RequestedItem::Invalid => serve_invalid_request(),
    };
    Ok(response)
}

fn determine_requested_item(request_path: &Path) -> RequestedItem {
    let first_component = request_path.components().skip(1).next();
    match first_component {
        Some(component) => match component {
            Component::RootDir | Component::Prefix(_) => panic!("Should not be possible"),
            Component::CurDir | Component::ParentDir => {
                return RequestedItem::Invalid;
            }
            Component::Normal(x) => {
                if x == STATIC_DIR_NAME || x == SCRIPT_DIR_NAME {
                    RequestedItem::StaticFile(request_path)
                } else {
                    RequestedItem::SPA
                }
            }
        },
        None => RequestedItem::SPA,
    }
}

fn serve_spa(app_root: &Path) -> Response<Body> {
    match fs::read_to_string(app_root.join(SPA_FILE_NAME)) {
        Ok(content) => {
            let mut resp = Response::new(content.into());
            let headers = resp.headers_mut();
            headers.append(
                "content-type",
                hyper::header::HeaderValue::from_str("text/html; charset=UTF-8").unwrap(),
            );
            resp
        }
        Err(e) => {
            let mut resp = Response::default();
            *resp.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
            eprintln!("Failed reading SPA file: {e}");
            resp
        }
    }
}

fn serve_static_file(path: &Path, app_root: &Path) -> Response<Body> {
    let path = path.strip_prefix("/").unwrap();
    match fs::read_to_string(app_root.join(path)) {
        Ok(content) => {
            let mut resp = Response::new(content.into());
            let headers = resp.headers_mut();
            headers.append(
                "content-type",
                hyper::header::HeaderValue::from_str(&mine_type_from_path(path)).unwrap(),
            );
            resp
        }
        Err(_e) => {
            let mut resp = Response::default();
            *resp.status_mut() = http::StatusCode::NOT_FOUND;
            resp
        }
    }
}

fn serve_invalid_request() -> Response<Body> {
    let mut resp = Response::new("Invalid request address".into());
    *resp.status_mut() = http::StatusCode::BAD_REQUEST;
    resp
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

fn mine_type_from_path(path: &Path) -> String {
    match mime_guess::from_path(path).first() {
        Some(v) => v.to_string(),
        None => "application/octet-stream".to_string(),
    }
}

pub async fn serve(options: ServerOptions) {
    let addr = SocketAddr::from(([0, 0, 0, 0], options.port as u16));

    let requested_app_root = Path::new(&options.app_root);
    let app_root_path = if requested_app_root.is_absolute() {
        requested_app_root.to_path_buf()
    } else {
        std::env::current_dir()
            .expect("Failed to find current dir")
            .join(requested_app_root)
    }
    .canonicalize()
    .unwrap();
    println!("Serving app from {}", app_root_path.to_string_lossy());
    let app_root = Arc::new(app_root_path);

    let make_svc = make_service_fn(move |_| {
        let app_root = app_root.clone();
        async move {
            Ok::<_, Infallible>(service_fn({
                move |req| {
                    let app_root = app_root.clone();
                    async move { serve_route(req, app_root).await }
                }
            }))
        }
    });

    let server = Server::bind(&addr)
        .serve(make_svc)
        .with_graceful_shutdown(shutdown_signal());

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
