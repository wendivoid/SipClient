use nirah_uri::Uri;

pub enum EncryptionKey {
    Clear(String),
    Base64(String),
    Uri(Uri),
    Prompt
}
