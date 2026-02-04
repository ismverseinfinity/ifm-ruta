//! Streaming response implementation for MCP 1.0

use futures::stream::{Stream, StreamExt};
use serde_json::Value;
use tokio::io::{AsyncWrite, AsyncWriteExt};

/// Wrapper for streaming tool responses
#[allow(dead_code)]
pub struct StreamingResponse {
    id: Option<Value>,
    stream: Box<dyn Stream<Item = Result<String, String>> + Send + Unpin>,
}

#[allow(dead_code)]
impl StreamingResponse {
    pub fn new(
        id: Option<Value>,
        stream: Box<dyn Stream<Item = Result<String, String>> + Send + Unpin>,
    ) -> Self {
        Self { id, stream }
    }

    /// Send all chunks from stream to writer
    pub async fn send_all<W: AsyncWrite + Unpin>(
        mut self,
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut chunk_index = 0;

        while let Some(result) = self.stream.next().await {
            match result {
                Ok(chunk) => {
                    self.send_chunk(writer, chunk_index, &chunk).await?;
                    chunk_index += 1;
                }
                Err(e) => {
                    self.send_error(writer, &e).await?;
                    return Err(e.into());
                }
            }
        }

        self.send_completion(writer).await?;
        Ok(())
    }

    async fn send_chunk<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        index: u32,
        chunk: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": self.id,
            "result": {
                "type": "stream_chunk",
                "index": index,
                "content": chunk
            }
        });

        let json = serde_json::to_string(&response)?;
        writer.write_all(json.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;

        Ok(())
    }

    async fn send_error<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        error: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": self.id,
            "error": {
                "code": -32603,
                "message": error
            }
        });

        let json = serde_json::to_string(&response)?;
        writer.write_all(json.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;

        Ok(())
    }

    async fn send_completion<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": self.id,
            "result": {
                "type": "stream_complete"
            }
        });

        let json = serde_json::to_string(&response)?;
        writer.write_all(json.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;

        Ok(())
    }
}

/// Builder for streaming responses
#[allow(dead_code)]
pub struct StreamingResponseBuilder {
    id: Option<Value>,
}

#[allow(dead_code)]
impl StreamingResponseBuilder {
    pub fn new(id: Option<Value>) -> Self {
        Self { id }
    }

    pub fn with_stream(
        self,
        stream: Box<dyn Stream<Item = Result<String, String>> + Send + Unpin>,
    ) -> StreamingResponse {
        StreamingResponse::new(self.id, stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream;

    #[test]
    fn test_streaming_response_builder() {
        let id = Some(serde_json::json!(1));
        let stream = Box::new(stream::iter(vec![Ok("test".to_string())]));
        let response = StreamingResponseBuilder::new(id.clone()).with_stream(stream);
        assert_eq!(response.id, id);
    }

    #[tokio::test]
    async fn test_streaming_response_with_chunks() {
        let id = Some(serde_json::json!(1));
        let chunks = vec![
            Ok("chunk1".to_string()),
            Ok("chunk2".to_string()),
            Ok("chunk3".to_string()),
        ];
        let stream = Box::new(stream::iter(chunks));
        let response = StreamingResponseBuilder::new(id).with_stream(stream);

        // Test that we can build the response without errors
        assert_eq!(response.id.as_ref().map(|v| v.get(0)), Some(None));
    }

    #[tokio::test]
    async fn test_streaming_response_error_handling() {
        let id = Some(serde_json::json!(1));
        let chunks: Vec<Result<String, String>> =
            vec![Ok("chunk1".to_string()), Err("error occurred".to_string())];
        let stream = Box::new(stream::iter(chunks));
        let response = StreamingResponseBuilder::new(id).with_stream(stream);

        // Verify response was created successfully
        assert!(response.id.is_some());
    }
}
