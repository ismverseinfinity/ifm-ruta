//! Tool registry integration tests

#[cfg(test)]
mod tool_registry_tests {
    use async_trait::async_trait;
    use ifm_ruta_core::services::ToolRegistry;
    use ifm_ruta_core::traits::async_tool::{AsyncTool, MCPResult, ToolMetadata, ToolResponse};
    use serde_json::{json, Value};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    // Test tool for testing
    struct TestTool {
        name: String,
        call_count: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl AsyncTool for TestTool {
        async fn execute(&self, args: Value) -> MCPResult<ToolResponse> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            let message = args
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("default");
            Ok(ToolResponse {
                content: format!("{}: {}", self.name, message),
                is_error: false,
            })
        }

        fn metadata(&self) -> ToolMetadata {
            ToolMetadata {
                name: self.name.clone(),
                description: format!("Test tool: {}", self.name),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "message": {"type": "string"}
                    }
                }),
                version: "1.0.0".to_string(),
            }
        }
    }

    #[tokio::test]
    async fn test_registry_creation() {
        let registry = ToolRegistry::new();
        assert_eq!(registry.tool_count().await, 0);
    }

    #[tokio::test]
    async fn test_single_tool_registration() {
        let registry = ToolRegistry::new();
        let call_count = Arc::new(AtomicUsize::new(0));
        let tool = Arc::new(TestTool {
            name: "test_tool".to_string(),
            call_count: call_count.clone(),
        });

        let result = registry.register_tool("test_tool", tool).await;
        assert!(result.is_ok());
        assert_eq!(registry.tool_count().await, 1);
    }

    #[tokio::test]
    async fn test_multiple_tool_registration() {
        let registry = ToolRegistry::new();

        for i in 0..5 {
            let call_count = Arc::new(AtomicUsize::new(0));
            let tool = Arc::new(TestTool {
                name: format!("tool_{}", i),
                call_count,
            });
            registry
                .register_tool(format!("tool_{}", i), tool)
                .await
                .expect("failed to register tool");
        }

        assert_eq!(registry.tool_count().await, 5);
    }

    #[tokio::test]
    async fn test_duplicate_registration_fails() {
        let registry = ToolRegistry::new();
        let call_count = Arc::new(AtomicUsize::new(0));
        let tool = Arc::new(TestTool {
            name: "duplicate_tool".to_string(),
            call_count,
        });

        let result1 = registry.register_tool("dup_tool", tool.clone()).await;
        assert!(result1.is_ok());

        let result2 = registry.register_tool("dup_tool", tool).await;
        assert!(result2.is_err());
    }

    #[tokio::test]
    async fn test_tool_retrieval() {
        let registry = ToolRegistry::new();
        let call_count = Arc::new(AtomicUsize::new(0));
        let tool = Arc::new(TestTool {
            name: "retrieve_tool".to_string(),
            call_count,
        });

        registry.register_tool("retrieve_tool", tool).await.unwrap();
        let retrieved = registry.get_tool("retrieve_tool").await;

        assert!(retrieved.is_ok());
    }

    #[tokio::test]
    async fn test_nonexistent_tool_retrieval_fails() {
        let registry = ToolRegistry::new();
        let result = registry.get_tool("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_tools() {
        let registry = ToolRegistry::new();

        for i in 0..3 {
            let call_count = Arc::new(AtomicUsize::new(0));
            let tool = Arc::new(TestTool {
                name: format!("list_tool_{}", i),
                call_count,
            });
            registry
                .register_tool(format!("list_tool_{}", i), tool)
                .await
                .unwrap();
        }

        let tools = registry.list_tools().await.unwrap();
        assert_eq!(tools.len(), 3);

        // Verify all tool names are present
        let names: Vec<_> = tools.iter().map(|t| t.name.clone()).collect();
        assert!(names.contains(&"list_tool_0".to_string()));
        assert!(names.contains(&"list_tool_1".to_string()));
        assert!(names.contains(&"list_tool_2".to_string()));
    }

    #[tokio::test]
    async fn test_get_tool_metadata() {
        let registry = ToolRegistry::new();
        let call_count = Arc::new(AtomicUsize::new(0));
        let tool = Arc::new(TestTool {
            name: "metadata_tool".to_string(),
            call_count,
        });

        registry.register_tool("metadata_tool", tool).await.unwrap();
        let metadata = registry.get_tool_metadata("metadata_tool").await.unwrap();

        assert_eq!(metadata.name, "metadata_tool");
        assert_eq!(metadata.version, "1.0.0");
        assert!(metadata.description.contains("metadata_tool"));
    }

    #[tokio::test]
    async fn test_tool_unregistration() {
        let registry = ToolRegistry::new();
        let call_count = Arc::new(AtomicUsize::new(0));
        let tool = Arc::new(TestTool {
            name: "unreg_tool".to_string(),
            call_count,
        });

        registry.register_tool("unreg_tool", tool).await.unwrap();
        assert!(registry.has_tool("unreg_tool").await);

        registry.unregister_tool("unreg_tool").await.unwrap();
        assert!(!registry.has_tool("unreg_tool").await);
    }

    #[tokio::test]
    async fn test_registry_clear() {
        let registry = ToolRegistry::new();

        for i in 0..3 {
            let call_count = Arc::new(AtomicUsize::new(0));
            let tool = Arc::new(TestTool {
                name: format!("clear_tool_{}", i),
                call_count,
            });
            registry
                .register_tool(format!("clear_tool_{}", i), tool)
                .await
                .unwrap();
        }

        assert_eq!(registry.tool_count().await, 3);
        registry.clear().await;
        assert_eq!(registry.tool_count().await, 0);
    }

    #[tokio::test]
    async fn test_tool_execution_tracking() {
        let registry = ToolRegistry::new();
        let call_count = Arc::new(AtomicUsize::new(0));
        let tool = Arc::new(TestTool {
            name: "exec_tool".to_string(),
            call_count: call_count.clone(),
        });

        registry
            .register_tool("exec_tool", tool.clone())
            .await
            .unwrap();

        let retrieved = registry.get_tool("exec_tool").await.unwrap();
        let result = retrieved.execute(json!({"message": "test"})).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.content.contains("exec_tool"));
        assert!(response.content.contains("test"));
    }

    #[tokio::test]
    async fn test_concurrent_tool_access() {
        let registry = Arc::new(ToolRegistry::new());
        let call_count = Arc::new(AtomicUsize::new(0));

        // Register a tool
        let tool = Arc::new(TestTool {
            name: "concurrent_tool".to_string(),
            call_count,
        });
        registry
            .register_tool("concurrent_tool", tool)
            .await
            .unwrap();

        // Spawn multiple concurrent access tasks
        let mut handles = vec![];
        for _ in 0..10 {
            let registry_clone = registry.clone();
            let handle = tokio::spawn(async move {
                let tool = registry_clone.get_tool("concurrent_tool").await;
                assert!(tool.is_ok());
            });
            handles.push(handle);
        }

        // Wait for all tasks
        for handle in handles {
            handle.await.unwrap();
        }

        assert_eq!(registry.tool_count().await, 1);
    }
}
