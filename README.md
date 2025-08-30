# elevenlabs_vc

[![Crates.io](https://img.shields.io/crates/v/elevenlabs_vc.svg)](https://crates.io/crates/elevenlabs_vc)
[![Docs.rs](https://docs.rs/elevenlabs_stt/badge.svg)](https://docs.rs/elevenlabs_vc)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

A type-safe, async Rust client for the [ElevenLabs Coice Changer API](https://elevenlabs.io/app/speech-synthesis/speech-to-speech). Transform audio from one voice to another. Maintain full control over emotion, timing and delivery. Ergonomic API.

## Features

- **Type-safe & Async**: Built with Rust's type system and async/await support
- **Builder Pattern**: Intuitive, chainable API for configuring VC requests
- **Predefined Voices**: Access to static voice definitions (`voices::all_voices::*`)
- **Model Support**: Full support for ElevenLabs models (`models::elevenlabs_models::*`)
- **Customizable**: Elevanlabs STT APIs, custom base URLs, and enterprise support
- **Tokio Ready**: Works seamlessly with the Tokio runtime
- **Audio Only**: Works with audios only, up to 50MB

## Check-out Also:

**This project is part of a milestone to implement all ElevenLabs APIs in Rust.**

- **[Elevenlabs TTS](https://crates.io/crates/elevenlabs_tts)**: ElevenLabs Text-to-Speech API. ✅
- **[Elevenlabs TTD](https://crates.io/crates/elevenlabs_ttd)**: ElevenLabs Text-to-Dialogue API. ✅
- **[Elevenlabs STT](https://crates.io/crates/elevenlabs_stt)**: ElevenLabs Speech-to-Text API. ✅
- **[Elevenlabs SFX](https://crates.io/crates/elevenlabs_sfx)**: ElevenLabs Sound Effects API. ✅
- **[Elevenlabs VC](https://crates.io/crates/elevenlabs_vc)**: ElevenLabs Voice Changer API. ✅
- **Elevenlabs TTV**: ElevenLabs Text-to-Voice API. ⏳
- **Elevenlabs CM**: ElevenLabs Music Compose API. ⏳
- **Elevenlabs AUI**: ElevenLabs Audio Isolation API. ⏳
- **Elevenlabs DUB**: ElevenLabs Dubbing API. ⏳

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
elevenlabs_vc = "0.0.1"
```

## Quick Start

```rust
use elevenlabs_vc::{ElevenLabsVCClient, voices};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ElevenLabsVCClient::new("your-api-key");

    let file_path = "inputs/speech.mp3";
    let file_content = std::fs::read(file_path)?;

    let audio = client.voice_changer([].to_vec(), voices::all_voices::PAUL.voice_id).execute().await?;

    std::fs::write("output.mp3", &audio)?;

    Ok(())
}
```

## Examples

### Basic Usage

```rust
use elevenlabs_vc::{ElevenLabsVCClient, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    let client = ElevenLabsVCClient::new(api_key);

    let file_path = "inputs/input.mp3";
    let file_content = std::fs::read(file_path)?;

    let audio = client
        .voice_changer(file_content, voices::all_voices::PAUL.voice_id)
        .execute()
        .await?;

    std::fs::create_dir_all("outputs")?;
    let audio_id = chrono::Utc::now().timestamp();
    let file_name = format!("outputs/{}.mp3", audio_id);
    std::fs::write(file_name.clone(), &audio)?;

    Ok(())
}
```

### Advanced Configuration

```rust
use elevenlabs_vc::{ElevenLabsVCClient, VoiceSettings, models, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    let client = ElevenLabsVCClient::new(api_key);

    let file_path = "inputs/input.mp3";
    let file_content = std::fs::read(file_path)?;

    let audio = client
        .voice_changer(file_content, voices::all_voices::PAUL.voice_id)
        .model(models::elevanlabs_models::ELEVEN_MULTILINGUAL_STS_V2)
        .remove_background_noise(true)
        .seed(2500)
        .voice_settings(
            VoiceSettings::default()
                .stability(0.5)
                .speaker_boost(true)
                .similarity_boost(0.7)
                .speed(0.97),
        )
        .execute()
        .await?;

    std::fs::create_dir_all("outputs")?;
    let audio_id = chrono::Utc::now().timestamp();
    let file_name = format!("outputs/{}.mp3", audio_id);
    std::fs::write(file_name.clone(), &audio)?;

    Ok(())
}
```

### Running Examples

```bash
# Set your API key
export ELEVENLABS_API_KEY=your_api_key_here

# Run the basic example
cargo run --example basic_vc

# Run the advanced example
cargo run --example advanced_vc
```

## API Overview

| Method                            | Description                                                               |
| --------------------------------- | ------------------------------------------------------------------------- |
| `ElevenLabsVCClient::new(String)` | Create client instance (required)\*                                       |
| `.output_format(String)`          | Output format of the changed voice audio (optional)                       |
| `.model_id(String)`               | Identifier of the model that will be used (optional)                      |
| `.voice_settings(VoiceSettings)`  | Voice settings overriding stored settings for the given voice (optional)  |
| `.seed(u32)`                      | Our system will make a best effort to sample deterministically (optional) |
| `.remove_background_noise(bool)`  | Remove the background noise from your audio input (optional)              |
| `.file_format(String)`            | The format of input audio (optional)                                      |
| `.execute()`                      | Run request → change audio voice (required)\*                             |

## Error Handling

The crate uses standard Rust error handling patterns. All async methods return `Result` types:

```rust
match client.voice_changer(audio, voice_id).execute().await {
    Ok(output) => println!("Audio changed voice: {}", output.len()),
    Err(e) => eprintln!("VC request failed: {}", e),
}
```

## Requirements

- Rust 1.70+ (for async/await support)
- Tokio runtime
- Valid ElevenLabs API key

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

## Contributing

Contributions are welcome! Please feel free to:

- Open issues for bugs or feature requests
- Submit pull requests with improvements
- Improve documentation or examples
- Add tests or benchmarks

Before contributing, please ensure your code follows Rust conventions and includes appropriate tests.

## Support

If you like this project, consider supporting me on Patreon 💖

[![Patreon](https://img.shields.io/badge/Support-Patreon-orange.svg)](https://www.patreon.com/elmarjanihamza/gift)

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

---

**Note**: This crate is not officially affiliated with ElevenLabs. Please refer to the [ElevenLabs API documentation](https://elevenlabs.io/docs/api-reference/speech-to-speech/convert) for the most up-to-date API information.
