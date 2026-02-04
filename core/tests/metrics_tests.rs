//! Metrics system integration tests

#[cfg(test)]
mod metrics_tests {
    use ifm_ruta_core::services::ToolMetrics;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_metrics_initialization() {
        let metrics = ToolMetrics::new();
        assert_eq!(metrics.call_count(), 0);
        assert_eq!(metrics.error_count(), 0);
        assert_eq!(metrics.success_count(), 0);
    }

    #[test]
    fn test_single_success_recording() {
        let metrics = ToolMetrics::new();
        metrics.record_success(Duration::from_millis(100));

        assert_eq!(metrics.call_count(), 1);
        assert_eq!(metrics.error_count(), 0);
        assert_eq!(metrics.success_count(), 1);
    }

    #[test]
    fn test_single_error_recording() {
        let metrics = ToolMetrics::new();
        metrics.record_error();

        assert_eq!(metrics.call_count(), 1);
        assert_eq!(metrics.error_count(), 1);
        assert_eq!(metrics.success_count(), 0);
    }

    #[test]
    fn test_mixed_success_and_errors() {
        let metrics = ToolMetrics::new();

        metrics.record_success(Duration::from_millis(100));
        metrics.record_success(Duration::from_millis(200));
        metrics.record_error();
        metrics.record_success(Duration::from_millis(150));

        assert_eq!(metrics.call_count(), 4);
        assert_eq!(metrics.error_count(), 1);
        assert_eq!(metrics.success_count(), 3);
    }

    #[test]
    fn test_duration_accumulation() {
        let metrics = ToolMetrics::new();

        metrics.record_success(Duration::from_millis(100));
        metrics.record_success(Duration::from_millis(200));
        metrics.record_success(Duration::from_millis(300));

        let stats = metrics.get_stats();
        assert_eq!(stats.total_duration, Duration::from_millis(600));
    }

    #[test]
    fn test_average_duration_calculation() {
        let metrics = ToolMetrics::new();

        metrics.record_success(Duration::from_millis(100));
        metrics.record_success(Duration::from_millis(200));
        metrics.record_success(Duration::from_millis(300));

        let stats = metrics.get_stats();
        assert_eq!(stats.average_duration, Duration::from_millis(200));
    }

    #[test]
    fn test_error_rate_calculation() {
        let metrics = ToolMetrics::new();

        metrics.record_success(Duration::from_millis(100));
        metrics.record_success(Duration::from_millis(100));
        metrics.record_error();
        metrics.record_error();

        let stats = metrics.get_stats();
        let error_rate = stats.error_rate();

        assert!((error_rate - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_success_rate_calculation() {
        let metrics = ToolMetrics::new();

        metrics.record_success(Duration::from_millis(100));
        metrics.record_success(Duration::from_millis(100));
        metrics.record_error();

        let stats = metrics.get_stats();
        let success_rate = stats.success_rate();

        assert!((success_rate - 66.66666666).abs() < 1.0);
    }

    #[test]
    fn test_zero_calls_error_rate() {
        let metrics = ToolMetrics::new();
        let stats = metrics.get_stats();

        assert_eq!(stats.error_rate(), 0.0);
        assert_eq!(stats.success_rate(), 100.0);
    }

    #[test]
    fn test_metrics_reset() {
        let metrics = ToolMetrics::new();

        metrics.record_success(Duration::from_millis(100));
        metrics.record_error();
        metrics.record_success(Duration::from_millis(200));

        assert_eq!(metrics.call_count(), 3);

        metrics.reset();

        assert_eq!(metrics.call_count(), 0);
        assert_eq!(metrics.error_count(), 0);
        assert_eq!(metrics.success_count(), 0);
    }

    #[test]
    fn test_duration_conversions() {
        let metrics = ToolMetrics::new();

        metrics.record_success(Duration::from_secs(1));
        metrics.record_success(Duration::from_millis(500));

        let stats = metrics.get_stats();
        assert!((stats.total_duration_secs() - 1.5).abs() < 0.01);
        assert!((stats.average_duration_ms() - 750.0).abs() < 1.0);
    }

    #[test]
    fn test_metrics_clone() {
        let metrics = ToolMetrics::new();
        metrics.record_success(Duration::from_millis(100));

        let cloned = metrics.clone();
        assert_eq!(cloned.call_count(), 1);
        assert_eq!(cloned.success_count(), 1);
    }

    #[tokio::test]
    async fn test_concurrent_metrics_updates() {
        let metrics = Arc::new(ToolMetrics::new());
        let mut handles = vec![];

        // Spawn multiple concurrent tasks that record metrics
        for _ in 0..100 {
            let metrics_clone = metrics.clone();
            let handle = tokio::spawn(async move {
                metrics_clone.record_success(Duration::from_millis(10));
            });
            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap();
        }

        assert_eq!(metrics.call_count(), 100);
        assert_eq!(metrics.success_count(), 100);
        assert_eq!(metrics.error_count(), 0);
    }

    #[tokio::test]
    async fn test_concurrent_mixed_operations() {
        let metrics = Arc::new(ToolMetrics::new());
        let mut handles = vec![];

        // Spawn tasks that do various operations
        for i in 0..50 {
            let metrics_clone = metrics.clone();
            let handle = tokio::spawn(async move {
                if i % 2 == 0 {
                    metrics_clone.record_success(Duration::from_millis(10 + i as u64));
                } else {
                    metrics_clone.record_error();
                }
            });
            handles.push(handle);
        }

        // Wait for all tasks
        for handle in handles {
            handle.await.unwrap();
        }

        assert_eq!(metrics.call_count(), 50);
        assert_eq!(metrics.error_count(), 25);
        assert_eq!(metrics.success_count(), 25);
    }

    #[test]
    fn test_metrics_stats_snapshot() {
        let metrics = ToolMetrics::new();

        metrics.record_success(Duration::from_millis(100));
        metrics.record_error();
        metrics.record_success(Duration::from_millis(200));

        let stats1 = metrics.get_stats();
        assert_eq!(stats1.call_count, 3);

        metrics.record_error();

        let stats2 = metrics.get_stats();
        assert_eq!(stats2.call_count, 4);

        // Ensure stats1 is independent
        assert_eq!(stats1.call_count, 3);
        assert_ne!(stats1.call_count, stats2.call_count);
    }

    #[test]
    fn test_100_percent_error_rate() {
        let metrics = ToolMetrics::new();

        for _ in 0..10 {
            metrics.record_error();
        }

        let stats = metrics.get_stats();
        assert_eq!(stats.call_count, 10);
        assert_eq!(stats.error_count, 10);
        assert_eq!(stats.success_count, 0);
        assert_eq!(stats.error_rate(), 100.0);
        assert_eq!(stats.success_rate(), 0.0);
    }

    #[test]
    fn test_100_percent_success_rate() {
        let metrics = ToolMetrics::new();

        for i in 0..10 {
            metrics.record_success(Duration::from_millis(100 * (i + 1) as u64));
        }

        let stats = metrics.get_stats();
        assert_eq!(stats.call_count, 10);
        assert_eq!(stats.error_count, 0);
        assert_eq!(stats.success_count, 10);
        assert_eq!(stats.error_rate(), 0.0);
        assert_eq!(stats.success_rate(), 100.0);
    }
}
