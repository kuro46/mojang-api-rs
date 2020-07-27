pub mod status;

use std::str::FromStr;
use std::vec::Vec;
use std::collections::BTreeMap;
use crate::status::*;

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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
