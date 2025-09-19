use std::sync::Arc;

use axum::extract::{
    State, WebSocketUpgrade,
    ws::{Message, WebSocket},
};
use futures::StreamExt;
use tracing::info;

use crate::{
    application::{
        audio_source::AudioSourceList, http::app_state::AppState, llm::LlmList, vad::VadList,
    },
    domain::{
        entities::{
            audio_buffer::AudioBuffer,
            audio_source_layer::{AudioSourceLayer, SendAudioCallback},
            history::history::History,
        },
        ports::audio_source::AudioSource,
        utils::Utils,
    },
    infrastructure::vad::local_vad::LocalVadAdapter,
};

pub async fn ws_twilio_handler(
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
            .find(|s| matches!(s, AudioSourceList::Twilio(_)))
            .cloned()
            .expect("No Twilio audio source found")
    };

    let llm = {
        let llms = state.llms.lock().await;
        llms.iter()
            .find(|s| matches!(s, LlmList::Gemini(_)))
            .cloned()
            .expect("No local audio source found")
    };

    let stt = {
        let stt = state.stt.lock().await;
        stt.clone()
    };

    let mut audio_source_layer = AudioSourceLayer {
        id: Utils::generate_uuid(),
        vad: &mut VadList::Local(LocalVadAdapter::new(1024)),
        stt: stt.clone(),
        llm: llm.clone(),
        pool_manager: state.pool_manager.clone(),
        history: &mut History::new(),
        audio_buffer: &mut AudioBuffer::new(),
        send_audio: SendAudioCallback::new({
            let audio_source = audio_source.clone();
            move |bytes| audio_source.send_audio(bytes)
        }),
    };

    info!("Nouvelle connexion Twilio id={}", audio_source_layer.id);
    while let Some(msg) = socket.next().await {
        if let Ok(Message::Text(message)) = msg {
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
