use std::convert::From;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

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

impl MojangServer {
    /// Returns `true` when a status for the server is informational.  
    /// Currently, status of some servers are always 'red'.
    pub fn is_used(&self) -> bool {
        match self {
            MojangServer::MinecraftNet => false,
            MojangServer::Session => true,
            MojangServer::Account => true,
            MojangServer::AuthServer => true,
            MojangServer::SessionServer => false,
            MojangServer::Api => true,
            MojangServer::Textures => true,
            MojangServer::MojangCom => false,
        }
    }

    /// Returns `true` when a status for the server is **not** informational.  
    /// See: [`is_used`]: #method.is_used
    pub fn is_unused(&self) -> bool {
        !self.is_used()
    }
}

impl fmt::Display for MojangServer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            MojangServer::MinecraftNet => "minecraft.net",
            MojangServer::Session => "session.minecraft.net",
            MojangServer::Account => "account.mojang.com",
            MojangServer::AuthServer => "authserver.mojang.com",
            MojangServer::SessionServer => "sessionserver.mojang.com",
            MojangServer::Api => "api.mojang.com",
            MojangServer::Textures => "textures.minecraft.net",
            MojangServer::MojangCom => "mojang.com",
        };
        write!(f, "{}", string)
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
            _ => return Err(StatusError::UnknownServer(s.to_string())),
        })
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ApiStatus {
    Green,
    Yellow,
    Red,
}

impl fmt::Display for ApiStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            ApiStatus::Red => "red",
            ApiStatus::Yellow => "yellow",
            ApiStatus::Green => "green",
        };
        write!(f, "{}", string)
    }
}

impl FromStr for ApiStatus {
    type Err = StatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "green" => ApiStatus::Green,
            "yellow" => ApiStatus::Yellow,
            "red" => ApiStatus::Red,
            _ => return Err(StatusError::UnknownStatus(s.to_string())),
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
