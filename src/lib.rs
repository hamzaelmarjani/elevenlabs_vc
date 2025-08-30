//! ElevenLabs Voice Changer API client
//!
//! A type-safe, async Rust client for the ElevenLabs TTS API.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use elevenlabs_vc::{ElevenLabsVCClient, models, voices};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ElevenLabsVCClient::new("your-api-key");
//!     
//!     let audio = client
//!         .voice_changer([].to_vec(), voices::all_voices::PAUL.voice_id)
//!         .execute()
//!         .await?;
//!     
//!     // audio is Vec<u8> - raw audio data
//!     std::fs::write("output.mp3", audio)?;
//!     Ok(())
//! }
//! ```

use reqwest::Client;

pub mod error;
pub mod models;
pub mod types;
pub mod voices;

pub use error::ElevenLabsVCError;
pub use types::*;

/// Main client for interacting with ElevenLabs API
#[derive(Clone)]
pub struct ElevenLabsVCClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ElevenLabsVCClient {
    /// Create a new ElevenLabs client with API key
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: "https://api.elevenlabs.io/v1".to_string(),
        }
    }

    /// Create a new client with custom base URL
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: base_url.into(),
        }
    }

    /// Start building a Voice Changer request
    pub fn voice_changer<F: Into<Vec<u8>>, S: Into<String>>(
        &self,
        audio: F,
        voice_id: S,
    ) -> VoiceChangerBuilder {
        VoiceChangerBuilder::new(self.clone(), audio.into(), voice_id.into())
    }

    /// Internal method to execute VC request
    pub(crate) async fn execute_vc(
        &self,
        request: VCRequest,
    ) -> Result<Vec<u8>, ElevenLabsVCError> {
        let mut form = reqwest::multipart::Form::new();

        let part = reqwest::multipart::Part::bytes(request.audio.clone())
            .file_name("audio")
            .mime_str("application/octet-stream")
            .map_err(|e| ElevenLabsVCError::RequestError(e));

        match part {
            Ok(part) => form = form.part("audio", part),
            Err(e) => return Err(e),
        }

        if let Some(voice_sttings) = request.voice_settings {
            let settings_json = serde_json::to_string(&voice_sttings)
                .map_err(|e| ElevenLabsVCError::SerializeError(e))?;
            form = form.text("voice_settings", settings_json);
        }

        let request_fields = vec![
            (
                "output_format",
                request.output_format.clone().map(|n| n.to_string()),
            ),
            ("model_id", request.model_id.map(|n| n.to_string())),
            ("seed", request.seed.map(|n| n.to_string())),
            (
                "remove_background_noise",
                request.remove_background_noise.map(|n| n.to_string()),
            ),
            ("file_format", request.file_format.map(|n| n.to_string())),
        ];

        for (key, value) in request_fields {
            if let Some(val) = value {
                form = form.text(key, val);
            }
        }

        let mut url = format!("{}/speech-to-speech/{}", self.base_url, request.voice_id);

        if let Some(o_format) = request.output_format {
            url = format!("{}/{}", url, o_format);
        };

        let response = self
            .client
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ElevenLabsVCError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        Ok(response.bytes().await?.to_vec())
    }
}

/// Builder for Voice Changer requests
pub struct VoiceChangerBuilder {
    client: ElevenLabsVCClient,
    audio: Vec<u8>,
    voice_id: String,
    output_format: Option<String>,
    model_id: Option<String>,
    voice_settings: Option<VoiceSettings>,
    seed: Option<u32>,
    remove_background_noise: Option<bool>,
    file_format: Option<String>,
}

impl VoiceChangerBuilder {
    fn new(client: ElevenLabsVCClient, audio: Vec<u8>, voice_id: String) -> Self {
        Self {
            client,
            audio,
            voice_id,
            output_format: None,
            model_id: None,
            voice_settings: None,
            seed: None,
            remove_background_noise: None,
            file_format: None,
        }
    }

    /// Set the output format to use
    pub fn output_format<S: Into<String>>(mut self, output_format: S) -> Self {
        self.output_format = Some(output_format.into());
        self
    }

    /// Set the model to use
    pub fn model<S: Into<String>>(mut self, model_id: S) -> Self {
        self.model_id = Some(model_id.into());
        self
    }

    /// Set voice settings (stability, similarity_boost, style, user_speaker_boost and speed).
    pub fn voice_settings(mut self, settings: VoiceSettings) -> Self {
        self.voice_settings = Some(settings);
        self
    }

    /// Set seeds to use
    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set remove background noise option
    pub fn remove_background_noise<B: Into<bool>>(mut self, remove_background_noise: B) -> Self {
        self.remove_background_noise = Some(remove_background_noise.into());
        self
    }

    /// Set the file format of input audio option
    pub fn file_format<S: Into<String>>(mut self, file_format: S) -> Self {
        self.file_format = Some(file_format.into());
        self
    }

    /// Execute the Voice Changer request
    pub async fn execute(self) -> Result<Vec<u8>, ElevenLabsVCError> {
        let request =
            VCRequest {
                audio: self.audio,
                voice_id: self.voice_id,
                model_id: Some(self.model_id.unwrap_or_else(|| {
                    models::elevanlabs_models::ELEVEN_ENGLISH_STS_V2.to_string()
                })), // Default to eleven_english_sts_v2
                output_format: self.output_format.or(None),
                seed: self.seed.or(None),
                voice_settings: self.voice_settings.or(None),
                remove_background_noise: self.remove_background_noise.or(None),
                file_format: self.file_format.or(None),
            };

        self.client.execute_vc(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = ElevenLabsVCClient::new("test-key");
        assert_eq!(client.api_key, "test-key");
    }

    #[test]
    fn test_builder_pattern() {
        let client = ElevenLabsVCClient::new("test-key");
        let builder = client.voice_changer([], "voice-123").model("model-456");

        // Builder pattern works
        assert_eq!(builder.audio.is_empty(), true);
        assert_eq!(builder.voice_id, "voice-123".to_string());
    }
}
