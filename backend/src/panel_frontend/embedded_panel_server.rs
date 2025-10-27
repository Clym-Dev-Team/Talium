use crate::axum::{convert_url, AxumState};
use axum::body::Body;
use axum::response::Response;
use http::Request;
use std::convert::Infallible;
use std::fs;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_http::body::Full;
use tower_http::services::{Redirect, ServeDir};
use tower_service::Service;
use url::Url;

const PANEL_DIST_DIR: &str = "panel_dist";
pub const SERVER_PANEL_PATH: &str = "/panel";

pub fn embedded_panel_service(state: AxumState) -> ServeDir<DynamicIndexHtmlHandlerService> {
    let dir_exists = fs::exists(PANEL_DIST_DIR);
    let index_exists = fs::exists(PANEL_DIST_DIR.to_owned() + "/index.html");
    if dir_exists.is_err() {
        eprintln!();
        eprintln!("Could not check existence of {} dir, error: {}", PANEL_DIST_DIR, dir_exists.unwrap_err());
        eprintln!("    Probably will not be able to server embedded panel");
        eprintln!();
    } else if index_exists.is_err() {
        eprintln!();
        eprintln!("Could not check existence of {}/index.html dir, error: {}", PANEL_DIST_DIR, index_exists.unwrap_err());
        eprintln!("    Probably will not be able to server embedded panel");
        eprintln!();
    } else if !dir_exists.unwrap() {
        eprintln!();
        eprintln!("Missing panel_dist, cannot server embedded panel!");
        eprintln!("Execute build_rust npm script to create the build dist for serving.");
        eprintln!("    in the panel project dir:> npm run build_rust");
        eprintln!("Alternatively, manually copy the generated dist of the build_rust npm script into the root of this project into {}. Or place it into {} next to this application, if you have a binary", PANEL_DIST_DIR, PANEL_DIST_DIR);
        eprintln!();
    } else if !index_exists.unwrap() {
        eprintln!("panel_dist is missing index.html cannot server embedded panel in a working state!");
    } else {
        let g = state.webserver_config.read().unwrap();
        println!("Hosting embedded panel at: {}panel", g.server_base_url);
    }
    ServeDir::new(PANEL_DIST_DIR)
        .append_index_html_on_directories(false)
        .fallback(DynamicIndexHtmlHandlerService::new(state))
}

pub fn to_panel_redirect(server_base_url: Url) -> Redirect<Full> {
    Redirect::<Full>::temporary(convert_url(server_base_url.join(SERVER_PANEL_PATH).unwrap()))
}

#[derive(Clone)]
pub struct DynamicIndexHtmlHandlerService {
    pub(crate) state: AxumState
}

impl DynamicIndexHtmlHandlerService {
    pub(crate) fn new(state: AxumState) -> DynamicIndexHtmlHandlerService {
        DynamicIndexHtmlHandlerService {
            state,
        }
    }
}

impl Service<Request<Body>> for DynamicIndexHtmlHandlerService {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        let index = fs::read_to_string(PANEL_DIST_DIR.to_owned() + "/index.html").unwrap();

        let c = {
            let g = self.state.webserver_config.read().unwrap();
            g.clone()
        };
        // strip trailing / in case of something like localhost:3487/ because it could interfere with creating paths by + "/somePath" in js
        let path_prefix = c.server_base_url.path().strip_suffix("/").unwrap_or(c.server_base_url.path());
        let bot_addr_without_trailing = c.server_base_url.as_str().strip_suffix("/").unwrap_or(c.server_base_url.as_str());
        let additional_attributes = format!(r#"<head panel_base_addr="{}{}" backend_base_addr="{}/bot" twitch_client_id="{}" "#, bot_addr_without_trailing, SERVER_PANEL_PATH, bot_addr_without_trailing, c.panel_auth_twitch_client_id);

        let new_index = index
            // We are expecting the paths in the build dist to already start with /panel (or more correctly the SERVER_PANEL_PATH we have set here)
            .replace("href=\"/", format!("href=\"{}/", path_prefix).as_str())
            .replace("src=\"/", format!("src=\"{}/", path_prefix).as_str())
            .replace("<head", additional_attributes.as_str());

        let body = Body::from(new_index);

        Box::pin(async {
            Ok(Response::builder().header("Content-Type", "text/html; charset=utf-8")
                .body(body)
                .unwrap())
        })
    }
}
