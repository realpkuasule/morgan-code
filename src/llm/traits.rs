use async_trait::async_trait;
use futures::stream::Stream;
use std::pin::Pin;
use crate::error::Result;
use super::types::{CompletionRequest, CompletionResponse, StreamChunk};

pub type CompletionStream = Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>;

#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;
    async fn stream(&self, request: CompletionRequest) -> Result<CompletionStream>;
    fn supports_tools(&self) -> bool;
    fn supports_streaming(&self) -> bool;
    fn model_name(&self) -> &str;
}
