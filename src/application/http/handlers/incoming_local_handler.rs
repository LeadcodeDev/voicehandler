use std::sync::Arc;

use axum::extract::{
    State, WebSocketUpgrade,
    ws::{Message, WebSocket},
};
use futures::StreamExt;
use tracing::info;

use crate::{
    application::{audio_source::AudioSourceList, http::app_state::AppState, vad::VadList},
    domain::{
        entities::{
            audio_buffer::AudioBuffer, audio_source_layer::AudioSourceLayer,
            history::history::History,
        },
        ports::audio_source::AudioSource,
        utils::Utils,
    },
    infrastructure::vad::local_vad::LocalVadAdapter,
};

pub async fn ws_local_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_twilio_socket(socket, state))
}

async fn handle_twilio_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let audio_source = {
        let audio_sources = state.audio_sources.lock().await;
        audio_sources
            .iter()
            .find(|s| matches!(s, AudioSourceList::Local(_)))
            .cloned()
            .expect("No local audio source found")
    };

    let stt = {
        let audio_sources = state.stt.lock().await;
        audio_sources.clone()
    };

    let _history = History::new();
    let mut audio_source_layer = AudioSourceLayer {
        id: Utils::generate_uuid(),
        vad: &mut VadList::Local(LocalVadAdapter::new(16000, 1365)),
        stt: stt.clone(),
        pool_manager: state.pool_manager.clone(),
        audio_buffer: &mut AudioBuffer::new(),
    };

    info!("Nouvelle connexion locale id={}", audio_source_layer.id);

    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(message) = msg {
            audio_source_layer
                .audio_buffer
                .override_streamed_buffer(message);

            let _ = audio_source.handle(&mut audio_source_layer).await;
        }
    }

    info!(
        "Connexion id={} fermée. Nettoyage des ressources.",
        audio_source_layer.id
    );
}
