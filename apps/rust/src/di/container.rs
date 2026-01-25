//! Service Container implementation for dependency injection.
//!
//! This provides a runtime-based DI container that stores type-erased services
//! and allows resolution by type. For compile-time DI, consider using traits directly.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A type-erased service container for runtime dependency injection.
///
/// # Example
///
/// ```ignore
/// use rustexpress::di::ServiceContainer;
///
/// // Define a service
/// struct DatabaseService {
///     connection_string: String,
/// }
///
/// // Register the service
/// let mut container = ServiceContainer::new();
/// container.register(DatabaseService {
///     connection_string: "postgres://localhost/db".to_string(),
/// });
///
/// // Resolve the service
/// let db = container.resolve::<DatabaseService>().unwrap();
/// ```
#[derive(Default)]
pub struct ServiceContainer {
    services: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl ServiceContainer {
    /// Create a new empty service container.
    pub fn new() -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
        }
    }

    /// Register a service instance.
    ///
    /// The service will be stored as a singleton and shared across all resolutions.
    pub fn register<T: Send + Sync + 'static>(&self, service: T) {
        let type_id = TypeId::of::<T>();
        let mut services = self.services.write().unwrap_or_else(|e| e.into_inner());
        services.insert(type_id, Arc::new(service));
    }

    /// Register a service as an Arc (for when you already have an Arc).
    pub fn register_arc<T: Send + Sync + 'static>(&self, service: Arc<T>) {
        let type_id = TypeId::of::<T>();
        let mut services = self.services.write().unwrap_or_else(|e| e.into_inner());
        services.insert(type_id, service);
    }

    /// Resolve a service by type.
    ///
    /// Returns `None` if the service is not registered.
    pub fn resolve<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();
        let services = self.services.read().unwrap_or_else(|e| e.into_inner());
        services
            .get(&type_id)
            .and_then(|s| s.clone().downcast::<T>().ok())
    }

    /// Check if a service is registered.
    pub fn has<T: Send + Sync + 'static>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        let services = self.services.read().unwrap_or_else(|e| e.into_inner());
        services.contains_key(&type_id)
    }
}

/// Trait for service providers that can register services into the container.
///
/// Implement this trait for modules that need to register their services.
///
/// # Example
///
/// ```ignore
/// use rustexpress::di::{ServiceContainer, ServiceProvider};
///
/// struct DatabaseServiceProvider;
///
/// impl ServiceProvider for DatabaseServiceProvider {
///     fn register(&self, container: &ServiceContainer) {
///         container.register(DatabaseConnection::new());
///         container.register(UserRepository::new());
///     }
/// }
/// ```
pub trait ServiceProvider: Send + Sync {
    /// Register services into the container.
    fn register(&self, container: &ServiceContainer);

    /// Boot the service provider after all services are registered.
    /// Override this for post-registration initialization.
    fn boot(&self, _container: &ServiceContainer) {}
}

/// Builder for ServiceContainer with fluent API.
pub struct ContainerBuilder {
    providers: Vec<Box<dyn ServiceProvider>>,
}

impl ContainerBuilder {
    /// Create a new container builder.
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    /// Add a service provider to be registered.
    pub fn with_provider<P: ServiceProvider + 'static>(mut self, provider: P) -> Self {
        self.providers.push(Box::new(provider));
        self
    }

    /// Build the container, registering all providers.
    pub fn build(self) -> ServiceContainer {
        let container = ServiceContainer::new();

        // Register phase
        for provider in &self.providers {
            provider.register(&container);
        }

        // Boot phase
        for provider in &self.providers {
            provider.boot(&container);
        }

        container
    }
}

impl Default for ContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestService {
        value: i32,
    }

    #[test]
    fn test_register_and_resolve() {
        let container = ServiceContainer::new();
        container.register(TestService { value: 42 });

        let resolved = container.resolve::<TestService>().unwrap();
        assert_eq!(resolved.value, 42);
    }

    #[test]
    fn test_resolve_nonexistent() {
        let container = ServiceContainer::new();
        let resolved = container.resolve::<TestService>();
        assert!(resolved.is_none());
    }

    #[test]
    fn test_has() {
        let container = ServiceContainer::new();
        assert!(!container.has::<TestService>());

        container.register(TestService { value: 1 });
        assert!(container.has::<TestService>());
    }
}
