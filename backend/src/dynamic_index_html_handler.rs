use crate::axum::AxumState;
use axum::body::Body;
use axum::response::Response;
use http::Request;
use std::convert::Infallible;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_service::Service;

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
        let index = std::fs::read_to_string("panel_dist/index.html").unwrap();

        let c = {
            let g = self.state.webserver_config.read().unwrap();
            g.clone()
        };
        // strip trailing / in case of something like localhost:3487/ because it could interfere with creating paths by + "/somePath" in js
        let path_prefix = c.server_base_url.path().strip_suffix("/").unwrap_or(c.server_base_url.path());
        let bot_addr_without_trailing = c.server_base_url.as_str().strip_suffix("/").unwrap_or(c.server_base_url.as_str());
        let additional_attributes = format!(r#"<head panel_base_addr="{}/panel" backend_base_addr="{}/bot" twitch_client_id="{}" "#, bot_addr_without_trailing, bot_addr_without_trailing, c.panel_auth_twitch_client_id);

        let new_index = index
            // We are expecting the paths in the build dist to already start with /panel
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
