#![allow(clippy::needless_update)]
use crate::Endpoint;

pub const MODERN_STYLE: u32 = 0;
pub const CLASSIC_STYLE: u32 = 1;

pub const DEFAULT_SERVERLIST_URL: &str =
    "NOT USED UNLESS SPECIFIED BY ADMIN=SERVERIP/serverlist.json";
pub const DEFAULT_MESSAGELIST_URL: &str =
    "NOT USED UNLESS SPECIFIED BY ADMIN=SERVERIP/messagelist.json";

pub fn get_default_endpoints() -> Vec<Endpoint> {
    vec![Endpoint {
        name: "Offline-Mode".into(),
        url: "OFFLINEMODE".into(),
        is_remote: true,
        ..Default::default()
    }]
}
