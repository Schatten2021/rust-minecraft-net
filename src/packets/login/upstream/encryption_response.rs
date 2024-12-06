use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x01]
pub struct EncryptionResponse {
    #[Var]
    pub shared_secret_length: i32,
    #[len = "shared_secret_length"]
    pub shared_secret: Vec<u8>,
    #[Var]
    pub verify_token_length: i32,
    #[len = "verify_token_length"]
    pub verify_token: Vec<u8>,
}
impl EncryptionResponse {
    pub fn new(secret: Vec<u8>, token: Vec<u8>) -> Self {
        Self {
            shared_secret_length: secret.len() as i32,
            shared_secret: secret,
            verify_token_length: token.len() as i32,
            verify_token: token,
        }
    }
}