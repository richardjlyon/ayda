use crate::anythingllm::client::AnythingLLMClient;
use crate::anythingllm::documents::{Document, DocumentsResponse, Item};
use crate::anythingllm::error::LLMError;

impl AnythingLLMClient {
    /// GET /documents
    pub async fn get_documents(&self) -> Result<Vec<Document>, LLMError> {
        let response = self.get("documents").await?.error_for_status()?;

        let documents_response = response.json::<DocumentsResponse>().await?;

        // documents_response is a nested list of folder and document types:
        // extract the documents from the response
        let mut documents: Vec<Document> = Vec::new();
        if let Some(items) = &documents_response.local_files.items {
            for item in items {
                process_item(item, &mut documents);
            }
        }

        Ok(documents)
    }
}

fn process_item(item: &Item, documents: &mut Vec<Document>) {
    if item.doc_type == "file" {
        documents.push(item.into());
    }

    if let Some(nested_items) = &item.items {
        for nested_item in nested_items {
            process_item(nested_item, documents);
        }
    }
}
