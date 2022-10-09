use std::convert::Infallible;
use std::fs;
use std::net::SocketAddr;
use std::path::{Component, Path};

use hyper::service::{make_service_fn, service_fn};
use hyper::{http, Body, Request, Response, Server};

const SPA_FILE_NAME: &'static str = "index.html";
const STATIC_DIR_NAME: &'static str = "static";

enum RequestedItem<'a> {
    SPA,
    StaticFile(&'a Path),
    Invalid,
}

async fn serve_route(req: Request<Body>, app_root: &Path) -> Result<Response<Body>, Infallible> {
    let path = Path::new(req.uri().path());
    let requested_item = determine_requested_item(&path);

    let response = match requested_item {
        RequestedItem::SPA => serve_spa(app_root),
        RequestedItem::StaticFile(file_path) => serve_static_file(&req, file_path, app_root),
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
                if x == STATIC_DIR_NAME {
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

fn serve_static_file(req: &Request<Body>, path: &Path, app_root: &Path) -> Response<Body> {
    match fs::read_to_string(app_root.join(path)) {
        Ok(content) => {
            let mut resp = Response::new(content.into());
            let content_type = req.headers().get("content-type");
            if let Some(header_value) = content_type {
                let headers = resp.headers_mut();
                headers.append("content-type", header_value.clone());
            }
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

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let app_root = std::env::current_dir().expect("Failed to find current dir");

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(move |req| serve_route(req, &app_root)))
    });

    let server = Server::bind(&addr)
        .serve(make_svc)
        .with_graceful_shutdown(shutdown_signal());

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
