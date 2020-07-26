use std::str::FromStr;
use std::string::ToString;
use thiserror::Error;
use std::vec::Vec;
use std::convert::From;
use std::collections::BTreeMap;

pub async fn retrieve_status() -> Result<BTreeMap<MojangServer, ApiStatus>, StatusError> {
    let strings = reqwest::get("https://status.mojang.com/check")
        .await?
        .json::<Vec<BTreeMap<String, String>>>()
        .await?;
    let mut converted = BTreeMap::new();
    for map in strings {
        if map.len() != 1 {
            panic!("Maybe mojang's format changed")
        }
        let (server, status) = map.iter().next().unwrap();
        converted.insert(MojangServer::from_str(&server)?, ApiStatus::from_str(&status)?);
    }
    Ok(converted)
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
pub enum MojangServer {
    /// minecraft.net
    MinecraftNet,
    /// session.minecraft.net
    Session,
    /// account.mojang.com
    Account,
    /// authserver.mojang.com
    AuthServer,
    /// sessionserver.mojang.com
    SessionServer,
    /// api.mojang.com
    Api,
    /// textures.minecraft.net
    Textures,
    /// mojang.com
    MojangCom,
}

impl ToString for MojangServer {
    fn to_string(&self) -> String {
        match self {
            MojangServer::MinecraftNet => "minecraft.net",
            MojangServer::Session => "session.minecraft.net",
            MojangServer::Account => "account.mojang.com",
            MojangServer::AuthServer => "authserver.mojang.com",
            MojangServer::SessionServer => "sessionserver.mojang.com",
            MojangServer::Api => "api.mojang.com",
            MojangServer::Textures => "textures.minecraft.net",
            MojangServer::MojangCom => "mojang.com",
        }.to_string()
    }
}

impl FromStr for MojangServer {
    type Err = StatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "minecraft.net" => MojangServer::MinecraftNet,
            "session.minecraft.net" => MojangServer::Session,
            "account.mojang.com" => MojangServer::Account,
            "authserver.mojang.com" => MojangServer::AuthServer,
            "sessionserver.mojang.com" => MojangServer::SessionServer,
            "api.mojang.com" => MojangServer::Api,
            "textures.minecraft.net" => MojangServer::Textures,
            "mojang.com" => MojangServer::MojangCom,
            _ => return Err(StatusError::UnknownServer(s.to_string()))
        })
    }
}

pub enum ApiStatus {
    Green,
    Yellow,
    Red,
}

impl ToString for ApiStatus {
    fn to_string(&self) -> String {
        match self {
            ApiStatus::Red => "red",
            ApiStatus::Yellow => "yellow",
            ApiStatus::Green => "green"
        }.to_string()
    }
}

impl FromStr for ApiStatus {
    type Err = StatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "green" => ApiStatus::Green,
            "yellow" => ApiStatus::Yellow,
            "red" => ApiStatus::Red,
            _ => return Err(StatusError::UnknownStatus(s.to_string()))
        })
    }
}

#[derive(Error, Debug)]
pub enum StatusError {
    #[error("produced value {0} is unknown status name")]
    UnknownStatus(String),
    #[error("produced value {0} is unknown server name")]
    UnknownServer(String),
    #[error("network error: {0}")]
    NetworkError(reqwest::Error),
}

impl From<reqwest::Error> for StatusError {
    fn from(error: reqwest::Error) -> Self {
        StatusError::NetworkError(error)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
