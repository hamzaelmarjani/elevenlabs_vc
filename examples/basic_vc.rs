use elevenlabs_vc::{ElevenLabsVCClient, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    // Creating ElevenLabs client
    let client = ElevenLabsVCClient::new(api_key);

    // Get audio file bytes
    let file_path = "inputs/input.mp3";
    let file_content = std::fs::read(file_path)?;

    // Run Voice Changer execution
    let audio = client
        .voice_changer(file_content, voices::all_voices::PAUL.voice_id)
        .execute()
        .await?;

    // Save to file to outputs directory
    std::fs::create_dir_all("outputs")?;
    let audio_id = chrono::Utc::now().timestamp();
    let file_name = format!("outputs/{}.mp3", audio_id);
    std::fs::write(file_name.clone(), &audio)?;
    println!("Audio saved to {}", file_name);

    Ok(())
}
