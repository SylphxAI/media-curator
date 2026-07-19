//! media-curator-cli — ADR-168 production authority cores.

pub mod cache_keys;
pub mod dedup;
pub mod discovery;
pub mod extensions;
pub mod file_stats;
pub mod hamming;
pub mod transfer;

use serde::Serialize;

/// Health probe body emitted by the `health` subcommand.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct HealthBody {
    pub status: &'static str,
    pub stub: bool,
    pub authority: &'static str,
}

/// Build the production health response.
#[must_use]
pub fn health_body() -> HealthBody {
    HealthBody {
        status: "ok",
        stub: false,
        authority: "rust",
    }
}

/// Serialize health JSON.
#[must_use]
pub fn health_json() -> String {
    match serde_json::to_string(&health_body()) {
        Ok(json) => json,
        Err(error) => panic!("serialize health json: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_body_is_ok_rust_authority() {
        let body = health_body();
        assert_eq!(body.status, "ok");
        assert!(!body.stub);
        assert_eq!(body.authority, "rust");
    }

    #[test]
    fn health_json_round_trips() {
        let json = health_json();
        assert!(json.contains(r#""status":"ok""#));
        assert!(json.contains(r#""authority":"rust""#));
    }
}
