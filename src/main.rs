use chatty_audio::helper::audio_codec::decrypt_url;

fn main() {
    let ciphertext = "ID2ieOjCrwfgWvL5sXl4B1ImC5QfbsDyboG4XrmNvTDrEDvrfcr0Mn/5QKRbmNP/KBfaFeyigA3duQuCdY/GmRw7tS9a8Gtq";
    let decrypted = decrypt_url(ciphertext);
    println!("{}", decrypted);
}
