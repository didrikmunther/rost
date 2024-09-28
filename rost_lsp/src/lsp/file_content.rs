use super::{util::LspRequest, LSPServer};
use lsp_types::DidChangeTextDocumentParams;
use lsp_types::DidOpenTextDocumentParams;
use lsp_types::Uri;
use std::collections::HashMap;
use std::error::Error;
use tokio::fs;
use tokio::sync::RwLock;

use super::util::strip_file_protocol;

#[derive(Debug)]
pub struct FileContent {
    pub content: String,
    pub version: Option<i32>,
}

impl FileContent {
    pub fn new(content: String) -> Self {
        Self {
            content,
            version: None,
        }
    }

    pub fn versioned(content: String, version: i32) -> Self {
        Self {
            content,
            version: Some(version),
        }
    }
}

#[derive(Debug)]
pub struct FileContents(RwLock<HashMap<Uri, FileContent>>);

impl FileContents {
    pub fn new() -> Self {
        Self(RwLock::new(HashMap::new()))
    }

    pub async fn update(&self, uri: Uri, content: FileContent) {
        self.0.write().await.insert(uri, content);
    }

    pub async fn update_from_file(&self, uri: Uri) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(strip_file_protocol(uri.as_str())).await?;
        self.update(uri, FileContent::new(content)).await;
        Ok(())
    }

    pub async fn get(&self, uri: &Uri) -> Result<String, Box<dyn std::error::Error>> {
        if !self.0.read().await.contains_key(uri) {
            self.update_from_file(uri.clone()).await?;
        }

        Ok(self.0.read().await.get(uri).unwrap().content.clone())
    }
}

impl Default for FileContents {
    fn default() -> Self {
        Self::new()
    }
}

impl LSPServer {
    pub async fn handle_did_change(
        &mut self,
        _request: &LspRequest,
        params: DidChangeTextDocumentParams,
    ) -> Result<(), Box<dyn Error>> {
        self.file_contents
            .update(
                params.text_document.uri.clone(),
                // TODO: Using full synchronisation, does the client only send one element?
                FileContent::versioned(
                    params.content_changes[0].text.clone(),
                    params.text_document.version,
                ),
            )
            .await;

        self.publish_diagnostics(
            &params.text_document.uri,
            Some(params.text_document.version),
        )
        .await?;

        Ok(())
    }

    pub async fn handle_did_open(
        &mut self,
        _request: &LspRequest,
        params: DidOpenTextDocumentParams,
    ) -> Result<(), Box<dyn Error>> {
        self.file_contents
            .update(
                params.text_document.uri.clone(),
                FileContent::versioned(
                    params.text_document.text.clone(),
                    params.text_document.version,
                ),
            )
            .await;

        self.publish_diagnostics(
            &params.text_document.uri,
            Some(params.text_document.version),
        )
        .await?;

        Ok(())
    }
}
