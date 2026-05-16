#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SessionRef {
    pub session_id: String,
    pub title: String,
}

impl SessionRef {
    pub fn new(
        session_id: impl Into<String>,
        title: impl Into<String>,
    ) -> anyhow::Result<SessionRef> {
        let session_id = session_id.into();
        if session_id.is_empty() {
            anyhow::bail!("session_id cannot be empty");
        }

        Ok(SessionRef {
            session_id,
            title: title.into(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteStatus {
    ServerDeleted,
    LocalDeleted,
    Partial,
    Failed,
    Undone,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DeleteResult {
    pub status: DeleteStatus,
    pub session_id: String,
    pub message: String,
    pub undo_token: Option<String>,
    pub backup_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportStatus {
    Exported,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExportResult {
    pub status: ExportStatus,
    pub session_id: String,
    pub message: String,
    pub filename: String,
    pub markdown: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_ref_new_rejects_empty_session_id() {
        let err = SessionRef::new("", "Untitled").unwrap_err();

        assert!(err.to_string().contains("session_id"));
    }

    #[test]
    fn session_ref_new_preserves_fields() {
        let session = SessionRef::new("session-123", "My Session").unwrap();

        assert_eq!(
            session,
            SessionRef {
                session_id: "session-123".to_string(),
                title: "My Session".to_string(),
            }
        );
    }
}
