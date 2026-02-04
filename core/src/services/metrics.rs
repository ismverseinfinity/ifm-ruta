//! Tool execution metrics and statistics collection

use parking_lot::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Tool execution metrics
#[derive(Clone)]
pub struct ToolMetrics {
    call_count: Arc<AtomicU64>,
    total_duration: Arc<Mutex<Duration>>,
    error_count: Arc<AtomicU64>,
}

impl ToolMetrics {
    pub fn new() -> Self {
        Self {
            call_count: Arc::new(AtomicU64::new(0)),
            total_duration: Arc::new(Mutex::new(Duration::ZERO)),
            error_count: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Record successful execution
    pub fn record_success(&self, duration: Duration) {
        self.call_count.fetch_add(1, Ordering::Relaxed);
        let mut total = self.total_duration.lock();
        *total += duration;
    }

    /// Record failed execution
    pub fn record_error(&self) {
        self.call_count.fetch_add(1, Ordering::Relaxed);
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Get metrics statistics
    pub fn get_stats(&self) -> MetricsStats {
        let call_count = self.call_count.load(Ordering::Relaxed);
        let error_count = self.error_count.load(Ordering::Relaxed);
        let total_duration = *self.total_duration.lock();

        let success_count = call_count.saturating_sub(error_count);
        let avg_duration = if call_count > 0 {
            total_duration / call_count as u32
        } else {
            Duration::ZERO
        };

        MetricsStats {
            call_count,
            error_count,
            success_count,
            total_duration,
            average_duration: avg_duration,
        }
    }

    /// Reset all metrics
    pub fn reset(&self) {
        self.call_count.store(0, Ordering::Relaxed);
        self.error_count.store(0, Ordering::Relaxed);
        *self.total_duration.lock() = Duration::ZERO;
    }

    /// Get call count
    pub fn call_count(&self) -> u64 {
        self.call_count.load(Ordering::Relaxed)
    }

    /// Get error count
    pub fn error_count(&self) -> u64 {
        self.error_count.load(Ordering::Relaxed)
    }

    /// Get success count
    pub fn success_count(&self) -> u64 {
        let call_count = self.call_count.load(Ordering::Relaxed);
        let error_count = self.error_count.load(Ordering::Relaxed);
        call_count.saturating_sub(error_count)
    }

    /// Get total duration
    pub fn total_duration(&self) -> Duration {
        *self.total_duration.lock()
    }
}

impl Default for ToolMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Metrics statistics snapshot
#[derive(Debug, Clone)]
pub struct MetricsStats {
    pub call_count: u64,
    pub error_count: u64,
    pub success_count: u64,
    pub total_duration: Duration,
    pub average_duration: Duration,
}

impl MetricsStats {
    /// Calculate error rate as a percentage
    pub fn error_rate(&self) -> f64 {
        if self.call_count == 0 {
            0.0
        } else {
            (self.error_count as f64 / self.call_count as f64) * 100.0
        }
    }

    /// Calculate success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        100.0 - self.error_rate()
    }

    /// Get total duration in seconds
    pub fn total_duration_secs(&self) -> f64 {
        self.total_duration.as_secs_f64()
    }

    /// Get average duration in milliseconds
    pub fn average_duration_ms(&self) -> f64 {
        self.average_duration.as_secs_f64() * 1000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let metrics = ToolMetrics::new();
        assert_eq!(metrics.call_count(), 0);
        assert_eq!(metrics.error_count(), 0);
        assert_eq!(metrics.success_count(), 0);
    }

    #[test]
    fn test_record_success() {
        let metrics = ToolMetrics::new();
        metrics.record_success(Duration::from_millis(100));

        assert_eq!(metrics.call_count(), 1);
        assert_eq!(metrics.error_count(), 0);
        assert_eq!(metrics.success_count(), 1);
    }

    #[test]
    fn test_record_error() {
        let metrics = ToolMetrics::new();
        metrics.record_success(Duration::from_millis(100));
        metrics.record_error();

        assert_eq!(metrics.call_count(), 2);
        assert_eq!(metrics.error_count(), 1);
        assert_eq!(metrics.success_count(), 1);
    }

    #[test]
    fn test_get_stats() {
        let metrics = ToolMetrics::new();
        metrics.record_success(Duration::from_millis(100));
        metrics.record_success(Duration::from_millis(200));
        metrics.record_error();

        let stats = metrics.get_stats();
        assert_eq!(stats.call_count, 3);
        assert_eq!(stats.error_count, 1);
        assert_eq!(stats.success_count, 2);
        assert_eq!(stats.total_duration, Duration::from_millis(300));
    }

    #[test]
    fn test_average_duration() {
        let metrics = ToolMetrics::new();
        metrics.record_success(Duration::from_millis(100));
        metrics.record_success(Duration::from_millis(200));
        metrics.record_success(Duration::from_millis(300));

        let stats = metrics.get_stats();
        assert_eq!(stats.total_duration, Duration::from_millis(600));
        assert_eq!(stats.average_duration, Duration::from_millis(200));
    }

    #[test]
    fn test_metrics_reset() {
        let metrics = ToolMetrics::new();
        metrics.record_success(Duration::from_millis(100));
        metrics.record_error();

        assert_eq!(metrics.call_count(), 2);

        metrics.reset();

        assert_eq!(metrics.call_count(), 0);
        assert_eq!(metrics.error_count(), 0);
        assert_eq!(metrics.success_count(), 0);
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
    }
}
