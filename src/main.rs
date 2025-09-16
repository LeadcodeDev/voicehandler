use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use axum::routing::get;
use axum_server::bind;
use clap::Parser;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use voicehanler_rs::{
    application::{
        audio_source::AudioSourceList,
        env::Args,
        http::{
            app_state::AppState,
            handlers::{
                incoming_local_handler::ws_local_handler,
                incoming_twilio_handler::ws_twilio_handler,
            },
        },
        stt::SttList,
    },
    domain::entities::pool::Pool,
    infrastructure::{
        audio_source::{local_source_adapter::LocalAdapter, twilio_source_adapter::TwilioAdapter},
        stt::scribe_adapter::ScribeAdapter,
    },
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let args = Arc::new(Args::parse());

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(args.logger.level.to_string())
        .with_writer(std::io::stderr);

    if args.logger.prettify {
        subscriber.json().init();
    } else {
        subscriber.init();
    }

    let trace_layer =
        TraceLayer::new_for_http().make_span_with(|request: &axum::extract::Request| {
            let uri: String = request.uri().to_string();
            info_span!("http_request", method = ?request.method(), uri)
        });

    let source_audio = vec![
        AudioSourceList::Twilio(TwilioAdapter::new()),
        AudioSourceList::Local(LocalAdapter::new()),
    ];

    let pool_manager = Pool::new();
    let state = Arc::new(AppState::new(
        pool_manager,
        SttList::Scribe(ScribeAdapter::new(
            args.elevenlabs.elevenlabs_api_key.clone(),
        )),
        source_audio,
    ));

    let app = Router::new()
        .route("/", get(ws_twilio_handler))
        .route("/local", get(ws_local_handler))
        .layer(trace_layer)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5050));
    println!("Listening on ws://{}", addr);

    bind(addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
