//! media-curator-cli — ADR-168 bounded differential slices (health + pipeline helpers).

pub mod discovery;
pub mod extensions;
pub mod file_stats;
pub mod hamming;

use serde::Serialize;

/// Health probe body emitted by the `health` subcommand.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct HealthBody {
    pub status: &'static str,
    pub stub: bool,
}

/// Build the S0 health response (dependency-free).
#[must_use]
pub fn health_body() -> HealthBody {
    HealthBody {
        status: "ok",
        stub: true,
    }
}

/// Serialize health JSON to stdout (no trailing newline required by contract).
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
    fn health_body_is_ok_stub() {
        let body = health_body();
        assert_eq!(body.status, "ok");
        assert!(body.stub);
    }

    #[test]
    fn health_json_round_trips() {
        let json = health_json();
        assert!(json.contains(r#""status":"ok""#));
        assert!(json.contains(r#""stub":true"#));
    }
}
