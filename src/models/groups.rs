use serde::{Deserialize, Serialize};
use sqlx::types::time::Date;

use crate::utils::traits::{Error, IntoSerial};

pub struct Groups {
    pub gid: i32,
    pub groupName: String,
    pub ownedBy: String,
}

impl IntoSerial for Groups {
    type Serial = GroupsSerial;

    fn to_serial(&self) -> Self::Serial {
        Self::Serial {
            gid: self.gid,
            groupName: self.groupName.clone(),
            ownedBy: self.ownedBy.clone()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GroupsSerial {
    pub gid: i32,
    pub groupName: String,
    pub ownedBy: String
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct GroupsQuery {
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateGroups {
    pub gid: i32,
    pub groupName: String,
    pub ownedBy: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateGroups {
    pub groupName: Option<String>,
    pub ownedBy: Option<String>
}

#[derive(Debug, Clone)]
pub struct GroupsError(pub String);
impl Error for GroupsError {
    fn new(err: String) -> Self {
        Self(err)
    }
}
impl std::fmt::Display for GroupsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to retrieve group due to the following error: {:#?}",
            self.0
        )
    }
}
