use serde::{Deserialize, Serialize};

use crate::schema::covers;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Cover {
    pub address: String,
    pub name: String
}
