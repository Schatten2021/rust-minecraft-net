use minecraft_net_proc::{Field, Packet};

#[derive(Debug, Field, Clone)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    #[when = "is_signed"]
    pub signature: Option<String>
}
impl Property {
    pub fn new(name: String, value: String, signature: Option<String>) -> Self {
        Self {
            name, 
            value,
            is_signed: signature.is_none(),
            signature,
        }
    }
}
#[derive(Debug, Packet, Clone)]
#[id = 0x02]
pub struct LoginSuccess {
    pub uuid: u128,
    pub username: String,
    #[Var]
    pub number_of_properties: i32,
    #[len = "number_of_properties"]
    pub properties: Vec<Property>,
    pub strict_error_handling: bool,
}
impl LoginSuccess {
    pub fn new(uuid: u128, username: String, properties: Vec<Property>, strict_error_handling: bool) -> Self {
        Self {
            uuid, 
            username,
            number_of_properties: properties.len() as i32,
            properties,
            strict_error_handling,
        }
    }
}