use serde::{Deserialize, Serialize};

// v1/workspaces
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspacesResponse {
    pub workspaces: Vec<WorkspacesResponseWorkspace>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspacesResponseWorkspace {
    pub id: u8,
    pub name: String,
    pub slug: String,
}
