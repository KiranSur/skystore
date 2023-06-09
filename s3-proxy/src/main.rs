use s3s::service::S3ServiceBuilder;

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

mod client_impls;
mod objstore_client;
mod skyproxy;
mod stream_utils;
mod type_utils;

use crate::skyproxy::SkyProxy;
#[tokio::main]
async fn main() {
    // Exit the process upon panic, this is used for debugging purpose.
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default_panic(info);
        std::process::exit(1);
    }));

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();

    // Setup our proxy object
    let proxy = SkyProxy::new().await;

    // Setup S3 service
    // TODO: Add auth and configure virtual-host style domain
    // https://github.com/Nugine/s3s/blob/b0b6878dafee0e08a876bec5239425fc40c01271/crates/s3s-fs/src/main.rs#L58-L66
    let s3_service = S3ServiceBuilder::new(proxy).build().into_shared();
    let service = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .on_request(|req: &hyper::Request<hyper::Body>, _span: &tracing::Span| {
                    info!("{} {} {:?}", req.method(), req.uri(), req.headers());
                })
                .on_response(
                    |res: &hyper::Response<s3s::Body>,
                     latency: std::time::Duration,
                     _span: &tracing::Span| {
                        info!(
                            "{} {}ms: {:?}",
                            res.status(),
                            latency.as_millis(),
                            res.body().bytes()
                        );
                    },
                ),
        )
        .service(s3_service);

    // Run server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8000));
    info!("Starting server on {}", addr);
    hyper::Server::bind(&addr)
        .serve(tower::make::Shared::new(service))
        .await
        .expect("server error");
}
