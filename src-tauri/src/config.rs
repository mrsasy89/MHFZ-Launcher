#![allow(clippy::needless_update)]

use crate::Endpoint;

pub const MODERN_STYLE: u32 = 0;
pub const CLASSIC_STYLE: u32 = 1;

pub const DEFAULT_SERVERLIST_URL: &str =
"NOT USED UNLESS SPECIFIED BY ADMIN=SERVERIP/serverlist.json";
pub const DEFAULT_MESSAGELIST_URL: &str =
"NOT USED UNLESS SPECIFIED BY ADMIN=SERVERIP/messagelist.json";

pub fn get_default_endpoints() -> Vec<Endpoint> {
    vec![
        Endpoint {
            url: "http://avalanchemhfz.ddns.net".into(),
            name: "Avalanche".into(),
            launcher_port: Some(9010),
            game_port: Some(53310),
            game_folder: None,
            version: mhf_iel::MhfVersion::ZZ,
            is_remote: true,
        },
        Endpoint {
            name: "Offline-Mode".into(),
            url: "OFFLINEMODE".into(),
            is_remote: true,
            ..Default::default()
        }
    ]
}
