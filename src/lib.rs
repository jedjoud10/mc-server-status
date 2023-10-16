mod tests;
use std::net::Ipv4Addr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Player {
    pub name: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Mod {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Players {
    pub online: u32,
    pub max: u32,
    pub list: Option<Vec<Player>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Plugin {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonRequest {
    pub online: bool,
    pub ip: String,
    pub port: u32,
    pub version: Option<String>,
    pub players: Option<Players>,
    pub mods: Option<Vec<Mod>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Protocol {
    pub version: u32,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SomethingWrapper<T: Clone + core::fmt::Debug> {
    pub raw: T,
    pub clean: T,
    pub html: T,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Map {
    pub raw: String,
    pub clean: String,
    pub html: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiDebug {
    pub ping: bool,
    pub query: bool,
    pub srv: bool,
    pub querymismatch: bool,
    pub ipinsrv: bool,
    pub cnameinsrv: bool,
    pub animatedmotd: bool,
    pub cachehit: bool,
    pub cachetime: u64,
    pub cacheexpire: u64,
    pub apiversion: u32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Server {
    pub online: bool,
    pub ip: String,
    pub port: u16,
    pub hostname: Option<String>,
    pub debug: Option<ApiDebug>,
    pub version: Option<String>,
    pub protocol: Option<Protocol>,
    pub icon: Option<String>,
    pub software: Option<String>,
    pub map: Option<SomethingWrapper<String>>,
    pub gamemode: Option<String>,
    pub serverid: Option<String>,
    pub eula_blocked: bool,
    pub motd: SomethingWrapper<Vec<String>>,
    pub players: Option<Players>,
    pub plugins: Option<Vec<Plugin>>,
    pub mods: Option<Vec<Mod>>,
    pub info: SomethingWrapper<Vec<String>>,
}

pub const URL: &str = "https://api.mcsrvstat.us/3/";

pub fn get_api_endpoint(ip: Ipv4Addr, port: Option<u16>) -> String {
    match port {
        Some(port) => format!("{URL}{ip}:{port}"),
        None => format!("{URL}{ip}"),
    }
}

pub async fn request(ip: Ipv4Addr, port: Option<u16>) -> reqwest::Result<Server> {
    reqwest::get(get_api_endpoint(ip, port))
        .await?
        .json::<Server>()
        .await
}