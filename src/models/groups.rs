use serde::{Deserialize, Serialize};

use crate::utils::traits::{Error, IntoSerial};

pub struct Groups {
    pub gid: i32,
    pub group_name: String,
    pub owned_by: String,
}

impl IntoSerial for Groups {
    type Serial = GroupsSerial;

    fn to_serial(&self) -> Self::Serial {
        Self::Serial {
            gid: self.gid,
            group_name: self.group_name.clone(),
            owned_by: self.owned_by.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GroupsSerial {
    pub gid: i32,
    pub group_name: String,
    pub owned_by: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct GroupsQuery {}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateGroups {
    pub gid: i32,
    pub group_name: String,
    pub owned_by: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateGroups {
    pub group_name: Option<String>,
    pub owned_by: Option<String>,
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
