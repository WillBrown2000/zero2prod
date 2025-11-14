pub mod health_check;
pub mod subscriptions;

// Re-export common route handlers for ergonomic use in startup.rs
pub use health_check::health_check;
pub use subscriptions::subscribe;

