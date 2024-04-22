//! Serves the web viewer wasm/html.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
//!

#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]

use std::{
    fmt::Display,
    net::SocketAddr,
    str::FromStr,
    task::{Context, Poll},
};

use futures_util::future;
use hyper::{server::conn::AddrIncoming, service::Service, Body, Request, Response};

#[cfg(feature = "sync")]
mod sync;
#[cfg(feature = "sync")]
pub use sync::WebViewerServerHandle;

pub const DEFAULT_WEB_VIEWER_SERVER_PORT: u16 = 9090;

// See `Cargo.toml` docs for the `__ci` feature for more information about the `disable_web_viewer_server` cfg:
#[cfg(not(any(disable_web_viewer_server, feature = "__ci")))]
mod data {
    #![allow(clippy::large_include_file)]

    // If you add/remove/change the paths here, also update the include-list in `Cargo.toml`!
    pub const INDEX_HTML: &[u8] = include_bytes!("../web_viewer/index.html");
    pub const FAVICON: &[u8] = include_bytes!("../web_viewer/favicon.svg");
    pub const SW_JS: &[u8] = include_bytes!("../web_viewer/sw.js");
    pub const VIEWER_JS: &[u8] = include_bytes!("../web_viewer/re_viewer.js");
    pub const VIEWER_WASM: &[u8] = include_bytes!("../web_viewer/re_viewer_bg.wasm");
}

/// Failure to host the web viewer.
#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum WebViewerServerError {
    #[error("Could not parse address: {0}")]
    AddrParseFailed(#[from] std::net::AddrParseError),

    #[error("Failed to bind to port {0} for HTTP: {1}")]
    BindFailed(WebViewerServerPort, hyper::Error),

    #[error("Failed to serve web viewer: {0}")]
    ServeFailed(hyper::Error),

    #[cfg(feature = "sync")]
    #[error("Failed to spawn web viewer thread: {0}")]
    ThreadSpawnFailed(#[from] std::io::Error),
}

struct Svc {
    // NOTE: Optional because it is possible to have the `analytics` feature flag enabled
    // while at the same time opting-out of analytics at run-time.
    #[cfg(feature = "analytics")]
    analytics: Option<re_analytics::Analytics>,
}

impl Svc {
    fn new() -> Self {
        Self {
            #[cfg(feature = "analytics")]
            analytics: match re_analytics::Analytics::new(std::time::Duration::from_secs(2)) {
                Ok(analytics) => Some(analytics),
                Err(err) => {
                    re_log::error!(%err, "failed to initialize analytics SDK");
                    None
                }
            },
        }
    }

    #[cfg(feature = "analytics")]
    fn on_serve_wasm(&self) {
        if let Some(analytics) = &self.analytics {
            analytics.record(re_analytics::event::ServeWasm);
        }
    }
}

impl Service<Request<Body>> for Svc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    #[cfg(any(disable_web_viewer_server, feature = "__ci"))]
    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        #[cfg(feature = "analytics")]
        if false {
            self.on_serve_wasm(); // to silence warning about the function being unused
        }

        // panic! is not enough in hyper (since it uses catch_unwind) - that only kills this thread. We want to quit.
        eprintln!("web_server compiled with '--cfg disable_web_viewer_server' or `--feature __ci` can't serve requests.");
        std::process::abort();
    }

    #[cfg(not(any(disable_web_viewer_server, feature = "__ci")))]
    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let response = Response::builder();

        let (mime, bytes) = match req.uri().path() {
            "/" | "/index.html" => ("text/html", data::INDEX_HTML),
            "/favicon.svg" => ("image/svg+xml", data::FAVICON),
            "/sw.js" => ("text/javascript", data::SW_JS),
            "/re_viewer.js" => ("text/javascript", data::VIEWER_JS),
            "/re_viewer_bg.wasm" => {
                #[cfg(feature = "analytics")]
                self.on_serve_wasm();
                ("application/wasm", data::VIEWER_WASM)
            }
            _ => {
                re_log::warn!("404 path: {}", req.uri().path());
                let body = Body::from(Vec::new());
                let rsp = response.status(404).body(body).unwrap();
                return future::ok(rsp);
            }
        };

        let body = Body::from(Vec::from(bytes));
        let mut response = response.status(200).body(body).unwrap();
        response.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            hyper::header::HeaderValue::from_static(mime),
        );
        future::ok(response)
    }
}

struct MakeSvc;

impl<T> Service<T> for MakeSvc {
    type Response = Svc;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        future::ok(Svc::new())
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Typed port for use with [`WebViewerServer`]
pub struct WebViewerServerPort(pub u16);

impl WebViewerServerPort {
    /// Port to use with [`WebViewerServer::new`] when you want the OS to pick a port for you.
    ///
    /// This is defined as `0`.
    pub const AUTO: Self = Self(0);
}

impl Default for WebViewerServerPort {
    fn default() -> Self {
        Self(DEFAULT_WEB_VIEWER_SERVER_PORT)
    }
}

impl Display for WebViewerServerPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Needed for clap
impl FromStr for WebViewerServerPort {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u16>() {
            Ok(port) => Ok(WebViewerServerPort(port)),
            Err(err) => Err(format!("Failed to parse port: {err}")),
        }
    }
}

/// HTTP host for the Rerun Web Viewer application
/// This serves the HTTP+Wasm+JS files that make up the web-viewer.
pub struct WebViewerServer {
    server: hyper::Server<AddrIncoming, MakeSvc>,
}

impl WebViewerServer {
    /// Create new [`WebViewerServer`] to host the Rerun Web Viewer on a specified port.
    ///
    /// [`WebViewerServerPort::AUTO`] will tell the OS choose any free port.
    ///
    /// ## Example
    /// ``` no_run
    /// # use re_web_viewer_server::{WebViewerServer, WebViewerServerPort, WebViewerServerError};
    /// # async fn example() -> Result<(), WebViewerServerError> {
    /// let server = WebViewerServer::new("0.0.0.0", WebViewerServerPort::AUTO)?;
    /// let server_url = server.server_url();
    /// server.serve().await?;
    /// # Ok(()) }
    /// ```
    pub fn new(bind_ip: &str, port: WebViewerServerPort) -> Result<Self, WebViewerServerError> {
        let bind_addr = format!("{bind_ip}:{port}").parse()?;

        let server = hyper::Server::try_bind(&bind_addr)
            .or_else(|hyper_err| {
                if hyper_err.to_string().contains("Address already in use") {
                    let fallback_addr = format!("{bind_ip}:0").parse().map_err(|_err| hyper_err)?;

                    hyper::Server::try_bind(&fallback_addr)
                } else {
                    Err(hyper_err)
                }
            })
            .map_err(|err| WebViewerServerError::BindFailed(port, err))?
            .serve(MakeSvc);

        Ok(Self { server })
    }

    pub async fn serve(self) -> Result<(), WebViewerServerError> {
        self.server
            .await
            .map_err(WebViewerServerError::ServeFailed)?;
        Ok(())
    }

    pub async fn serve_with_graceful_shutdown(
        self,
        shutdown_rx: futures_channel::oneshot::Receiver<()>,
    ) -> Result<(), WebViewerServerError> {
        self.server
            .with_graceful_shutdown(async {
                shutdown_rx.await.ok();
            })
            .await
            .map_err(WebViewerServerError::ServeFailed)?;
        Ok(())
    }

    /// Includes `http://` prefix
    pub fn server_url(&self) -> String {
        server_url(&self.server.local_addr())
    }
}

pub(crate) fn server_url(local_addr: &SocketAddr) -> String {
    if local_addr.ip().is_unspecified() {
        // "0.0.0.0"
        format!("http://localhost:{}", local_addr.port())
    } else {
        format!("http://{local_addr}")
    }
}
