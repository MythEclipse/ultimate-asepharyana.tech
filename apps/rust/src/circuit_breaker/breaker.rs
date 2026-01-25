//! Circuit breaker implementation.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Circuit breaker state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircuitState {
    /// Circuit is closed (normal operation).
    Closed,
    /// Circuit is open (failing, rejecting requests).
    Open,
    /// Circuit is half-open (testing if service recovered).
    HalfOpen,
}

/// Circuit breaker configuration.
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening the circuit.
    pub failure_threshold: u32,
    /// Duration the circuit stays open before testing.
    pub reset_timeout: Duration,
    /// Number of successful calls in half-open to close.
    pub success_threshold: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            reset_timeout: Duration::from_secs(30),
            success_threshold: 2,
        }
    }
}

/// Circuit breaker for protecting external service calls.
///
/// # Example
///
/// ```ignore
/// let breaker = CircuitBreaker::new("external_api", CircuitBreakerConfig::default());
///
/// // Wrap your service call
/// let result = breaker.call(|| async {
///     client.get("https://api.example.com/data").await
/// }).await;
/// ```
pub struct CircuitBreaker {
    name: String,
    config: CircuitBreakerConfig,
    state: RwLock<CircuitState>,
    failure_count: AtomicU32,
    success_count: AtomicU32,
    last_failure_time: RwLock<Option<Instant>>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker.
    pub fn new(name: &str, config: CircuitBreakerConfig) -> Arc<Self> {
        Arc::new(Self {
            name: name.to_string(),
            config,
            state: RwLock::new(CircuitState::Closed),
            failure_count: AtomicU32::new(0),
            success_count: AtomicU32::new(0),
            last_failure_time: RwLock::new(None),
        })
    }

    /// Get current circuit state.
    pub async fn state(&self) -> CircuitState {
        *self.state.read().await
    }

    /// Execute a call through the circuit breaker.
    pub async fn call<F, Fut, T, E>(&self, f: F) -> Result<T, CircuitBreakerError<E>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        // Check if circuit should transition from Open to HalfOpen
        self.check_transition().await;

        let state = *self.state.read().await;

        match state {
            CircuitState::Open => {
                warn!("Circuit breaker '{}' is OPEN, rejecting request", self.name);
                Err(CircuitBreakerError::CircuitOpen)
            }
            CircuitState::Closed | CircuitState::HalfOpen => match f().await {
                Ok(result) => {
                    self.record_success().await;
                    Ok(result)
                }
                Err(e) => {
                    self.record_failure().await;
                    Err(CircuitBreakerError::ServiceError(e))
                }
            },
        }
    }

    async fn check_transition(&self) {
        let state = *self.state.read().await;
        if state == CircuitState::Open {
            let last_failure = self.last_failure_time.read().await;
            if let Some(time) = *last_failure {
                if time.elapsed() >= self.config.reset_timeout {
                    info!("Circuit breaker '{}' transitioning to HALF_OPEN", self.name);
                    *self.state.write().await = CircuitState::HalfOpen;
                    self.success_count.store(0, Ordering::SeqCst);
                }
            }
        }
    }

    async fn record_success(&self) {
        let state = *self.state.read().await;

        match state {
            CircuitState::Closed => {
                // Reset failure count on success
                self.failure_count.store(0, Ordering::SeqCst);
            }
            CircuitState::HalfOpen => {
                let count = self.success_count.fetch_add(1, Ordering::SeqCst) + 1;
                if count >= self.config.success_threshold {
                    info!(
                        "Circuit breaker '{}' closing after {} successes",
                        self.name, count
                    );
                    *self.state.write().await = CircuitState::Closed;
                    self.failure_count.store(0, Ordering::SeqCst);
                }
            }
            CircuitState::Open => {}
        }
    }

    async fn record_failure(&self) {
        let state = *self.state.read().await;

        match state {
            CircuitState::Closed => {
                let count = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;
                if count >= self.config.failure_threshold {
                    warn!(
                        "Circuit breaker '{}' opening after {} failures",
                        self.name, count
                    );
                    *self.state.write().await = CircuitState::Open;
                    *self.last_failure_time.write().await = Some(Instant::now());
                }
            }
            CircuitState::HalfOpen => {
                warn!(
                    "Circuit breaker '{}' reopening after failure in half-open",
                    self.name
                );
                *self.state.write().await = CircuitState::Open;
                *self.last_failure_time.write().await = Some(Instant::now());
            }
            CircuitState::Open => {}
        }
    }
}

/// Error type for circuit breaker operations.
#[derive(Debug)]
pub enum CircuitBreakerError<E> {
    /// Circuit is open, request was rejected.
    CircuitOpen,
    /// Underlying service returned an error.
    ServiceError(E),
}

impl<E: std::fmt::Display> std::fmt::Display for CircuitBreakerError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitBreakerError::CircuitOpen => write!(f, "Circuit breaker is open"),
            CircuitBreakerError::ServiceError(e) => write!(f, "Service error: {}", e),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for CircuitBreakerError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CircuitBreakerError::ServiceError(e) => Some(e),
            _ => None,
        }
    }
}
