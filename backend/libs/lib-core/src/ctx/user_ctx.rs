use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// User context for authenticated requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCtx {
    pub user_id: Uuid,
    pub email: String,
}

impl UserCtx {
    pub fn new(user_id: Uuid, email: String) -> Self {
        Self { user_id, email }
    }
}
