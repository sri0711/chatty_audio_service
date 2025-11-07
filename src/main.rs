use chatty_audio::{app /* helper::audio_codec::decrypt_url */};

#[tokio::main]
async fn main() {
    // let ciphertext = "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyboG4XrmNvTDrEDvrfcr0Mn/5QKRbmNP/KBfaFeyigA3duQuCdY/GmRw7tS9a8Gtq";
    // let decrypted = decrypt_url(ciphertext);
    // println!("{}", decrypted);

    std::panic::set_hook(Box::new(|info| {
        eprintln!("PANIC: {}", info);
    }));

    app::run().await
}
