#[derive(Debug, serde::Deserialize, Clone)]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct GetWorkspacesResponse {
    pub workspaces: Vec<Workspace>,
}

#[derive(Debug, serde::Deserialize)]
pub struct GetWorkspaceNewResponse {
    pub workspace: Workspace,
    pub message: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct GetWorkspaceSlugResponse {
    pub workspace: Workspace,
}
