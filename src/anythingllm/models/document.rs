use serde::{Deserialize, Serialize};

// v1/document/upload
#[derive(Debug, Deserialize)]
pub struct DocumentUploadResponse {
    pub success: bool,
    pub error: Option<String>,
    pub documents: Vec<DocumentUploadResponseDocuments>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DocumentUploadResponseDocuments {
    pub id: String,
    pub title: String,
}

impl DocumentUploadResponseDocuments {
    /// Synthesise the internal name of a document from title and id
    pub fn name_internal(&self) -> String {
        format!("{}-{}.json", self.title, self.id)
    }

    /// Synthesise the internal filepath of a document
    pub fn doc_filepath_internal(&self) -> String {
        format!("custom-documents/{}", self.name_internal())
    }
}

// v1/documents

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentsResponse {
    #[serde(rename = "localFiles")]
    pub local_files: DocumentsResponseLocalFiles,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentsResponseLocalFiles {
    pub items: Vec<DocumentsResponseItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentsResponseItem {
    pub name: String,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub item_type: String,
    pub items: Option<Vec<DocumentsResponseItem>>,
    pub title: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_name() {
        let doc = DocumentUploadResponseDocuments {
            id: "c516372a-24fc-48b6-9be5-85ace8ec2e28".to_string(),
            title: "Addiscott-2012-Climate-Change-and-Modelling.pdf".to_string(),
        };
        let expected_name_internal = "Addiscott-2012-Climate-Change-and-Modelling.pdf-c516372a-24fc-48b6-9be5-85ace8ec2e28.json";
        let expected_doc_filepath_internal = format!("custom-documents/{}", expected_name_internal);
        assert_eq!(doc.name_internal(), expected_name_internal);
        assert_eq!(doc.doc_filepath_internal(), expected_doc_filepath_internal);
    }
}
