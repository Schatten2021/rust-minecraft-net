use minecraft_net_proc::Packet;
#[derive(Debug, Packet)]
#[id = 0x01]
pub struct EncryptionRequest {
    pub server_id: String,
    #[Var]
    pub public_key_length: i32,
    #[len = "public_key_length"]
    pub public_key: Vec<u8>,
    #[Var]
    pub verify_token_length: i32,
    #[len = "verify_token_length"]
    pub verify_token: Vec<u8>,
    pub should_authenticate: bool,
}
impl EncryptionRequest {
    pub fn new(server_id: String, public_key: Vec<u8>, verify_token: Vec<u8>, should_authenticate: bool) -> Self {
        Self {
            server_id,
            public_key_length: public_key.len() as i32,
            public_key,
            verify_token_length: verify_token.len() as i32,
            verify_token,
            should_authenticate,
        }
    }
}