use daichi::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleToggle {
    role_toggle: bool,
    pub role_id: serenity::RoleId,
}

impl RoleToggle {
    pub fn new(role_id: serenity::RoleId) -> Self {
        Self {
            role_toggle: true,
            role_id,
        }
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Error::from_any)
    }

    pub fn from_json(value: &str) -> Result<Self> {
        serde_json::from_str(value).map_err(Error::from_any)
    }
}
