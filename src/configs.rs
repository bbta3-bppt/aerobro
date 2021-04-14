use std::env;
use crate::errors::AppErrors;


#[derive(Debug, Clone)]
pub struct AppConfigs {
    addr: String,
    user: String,
    password: String,
    topic: String,
    websocket: String
}

impl AppConfigs {
    fn new(addr: String, user: String, password: String, topic: String, websocket: String) -> Self {
        Self { addr, user, password, topic, websocket }
    }

    pub fn get_addr(&self) -> String {
        self.addr.clone()
    }

    pub fn get_user(&self) -> String {
        self.user.clone()
    }

    pub fn get_password(&self) -> String {
        self.password.clone()
    }

    pub fn get_topic(&self) -> String {
        self.topic.clone()
    }

    pub fn get_websocket_addr(&self) -> String {
        self.websocket.clone()
    }
}

pub fn get_configs() -> Result<AppConfigs, AppErrors> {
    let addr = env::var("MQTT_ADDR")?;
    let user = env::var("MQTT_USER")?;
    let password = env::var("MQTT_PWD")?;
    let topic = env::var("MSG_TOPIC")?;
    let websocket = env::var("SOCKET_ADDRESS")?;
    let configs = AppConfigs::new(addr, user, password, topic, websocket);

    Ok(configs)
}