/// Session store — per-session widget state management.
///
/// Each browser tab gets its own isolated session (UUID assigned on first connection).
/// Session state is in-memory; lost on server restart (by design).
/// Sessions expire after a configurable TTL (default: 24 hours of inactivity).
use dashmap::DashMap;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::vdom::VNode;

/// Default session time-to-live: 24 hours of inactivity.
const DEFAULT_SESSION_TTL: Duration = Duration::from_secs(24 * 60 * 60);

/// Represents a single user session.
#[derive(Debug, Clone)]
pub struct Session {
    /// Unique session identifier.
    pub id: Uuid,
    /// Widget state: maps widget_id -> current JSON value.
    pub widget_state: HashMap<String, Value>,
    /// User-defined session state: maps key -> JSON value.
    pub user_state: HashMap<String, Value>,
    /// Last rendered VNode tree (for diffing).
    pub last_tree: Option<VNode>,
    /// Timestamp of the last activity (for TTL expiration).
    pub last_active: Instant,
}

impl Session {
    /// Create a new session with a unique ID.
    pub fn new() -> Self {
        Session {
            id: Uuid::new_v4(),
            widget_state: HashMap::new(),
            user_state: HashMap::new(),
            last_tree: None,
            last_active: Instant::now(),
        }
    }

    /// Update the last activity timestamp.
    pub fn touch(&mut self) {
        self.last_active = Instant::now();
    }

    /// Check if this session has expired given a TTL duration.
    pub fn is_expired(&self, ttl: Duration) -> bool {
        self.last_active.elapsed() > ttl
    }

    /// Get widget value, returning default if not set.
    pub fn get_widget_value(&self, widget_id: &str, default: Value) -> Value {
        self.widget_state.get(widget_id).cloned().unwrap_or(default)
    }

    /// Set widget value.
    pub fn set_widget_value(&mut self, widget_id: &str, value: Value) {
        self.widget_state.insert(widget_id.to_string(), value);
    }

    /// Get user state value.
    pub fn get_state(&self, key: &str) -> Option<&Value> {
        self.user_state.get(key)
    }

    /// Set user state value.
    pub fn set_state(&mut self, key: &str, value: Value) {
        self.user_state.insert(key.to_string(), value);
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe session store backed by DashMap.
#[derive(Debug, Clone)]
pub struct SessionStore {
    sessions: Arc<DashMap<Uuid, Session>>,
    /// TTL for session expiration (default: 24 hours).
    ttl: Duration,
}

impl SessionStore {
    /// Create a new empty session store with default TTL (24 hours).
    pub fn new() -> Self {
        SessionStore {
            sessions: Arc::new(DashMap::new()),
            ttl: DEFAULT_SESSION_TTL,
        }
    }

    /// Create a new session store with a custom TTL.
    pub fn with_ttl(ttl: Duration) -> Self {
        SessionStore {
            sessions: Arc::new(DashMap::new()),
            ttl,
        }
    }

    /// Create a new session and return its ID.
    pub fn create_session(&self) -> Uuid {
        let session = Session::new();
        let id = session.id;
        self.sessions.insert(id, session);
        id
    }

    /// Get a reference to a session by ID. Panics are not generated;
    /// returns None for expired or non-existent sessions.
    pub fn get_session(&self, id: &Uuid) -> Option<dashmap::mapref::one::Ref<'_, Uuid, Session>> {
        let session_ref = self.sessions.get(id)?;
        if session_ref.is_expired(self.ttl) {
            drop(session_ref);
            self.sessions.remove(id);
            return None;
        }
        Some(session_ref)
    }

    /// Get a mutable reference to a session by ID.
    /// Automatically touches (updates last_active) the session.
    pub fn get_session_mut(
        &self,
        id: &Uuid,
    ) -> Option<dashmap::mapref::one::RefMut<'_, Uuid, Session>> {
        let mut session_ref = self.sessions.get_mut(id)?;
        if session_ref.is_expired(self.ttl) {
            drop(session_ref);
            self.sessions.remove(id);
            return None;
        }
        session_ref.touch();
        Some(session_ref)
    }

    /// Remove a session by ID.
    pub fn remove_session(&self, id: &Uuid) -> Option<Session> {
        self.sessions.remove(id).map(|(_, s)| s)
    }

    /// Get the number of active sessions.
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    /// Remove all expired sessions. Returns the number of sessions removed.
    pub fn cleanup_expired(&self) -> usize {
        let ttl = self.ttl;
        let expired_ids: Vec<Uuid> = self
            .sessions
            .iter()
            .filter(|entry| entry.value().is_expired(ttl))
            .map(|entry| *entry.key())
            .collect();
        let count = expired_ids.len();
        for id in expired_ids {
            self.sessions.remove(&id);
        }
        count
    }

    /// Get the configured TTL.
    pub fn ttl(&self) -> Duration {
        self.ttl
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_session() {
        let store = SessionStore::new();
        let id = store.create_session();
        assert!(store.get_session(&id).is_some());
        assert_eq!(store.session_count(), 1);
    }

    #[test]
    fn test_widget_state() {
        let store = SessionStore::new();
        let id = store.create_session();

        {
            let mut session = store.get_session_mut(&id).unwrap();
            session.set_widget_value("slider1", serde_json::json!(42));
        }

        let session = store.get_session(&id).unwrap();
        assert_eq!(
            session.get_widget_value("slider1", serde_json::json!(0)),
            serde_json::json!(42)
        );
    }

    #[test]
    fn test_user_state() {
        let store = SessionStore::new();
        let id = store.create_session();

        {
            let mut session = store.get_session_mut(&id).unwrap();
            session.set_state("counter", serde_json::json!(5));
        }

        let session = store.get_session(&id).unwrap();
        assert_eq!(session.get_state("counter"), Some(&serde_json::json!(5)));
    }

    #[test]
    fn test_remove_session() {
        let store = SessionStore::new();
        let id = store.create_session();
        assert_eq!(store.session_count(), 1);
        store.remove_session(&id);
        assert_eq!(store.session_count(), 0);
        assert!(store.get_session(&id).is_none());
    }

    #[test]
    fn test_multiple_sessions_isolated() {
        let store = SessionStore::new();
        let id1 = store.create_session();
        let id2 = store.create_session();

        {
            let mut s1 = store.get_session_mut(&id1).unwrap();
            s1.set_widget_value("slider", serde_json::json!(10));
        }
        {
            let mut s2 = store.get_session_mut(&id2).unwrap();
            s2.set_widget_value("slider", serde_json::json!(99));
        }

        let s1 = store.get_session(&id1).unwrap();
        let s2 = store.get_session(&id2).unwrap();
        assert_eq!(
            s1.get_widget_value("slider", serde_json::json!(0)),
            serde_json::json!(10)
        );
        assert_eq!(
            s2.get_widget_value("slider", serde_json::json!(0)),
            serde_json::json!(99)
        );
    }

    #[test]
    fn test_default_widget_value() {
        let session = Session::new();
        let val = session.get_widget_value("nonexistent", serde_json::json!("default"));
        assert_eq!(val, serde_json::json!("default"));
    }

    #[test]
    fn test_session_touch() {
        let mut session = Session::new();
        let first_active = session.last_active;
        // Small delay to ensure Instant differs
        std::thread::sleep(Duration::from_millis(10));
        session.touch();
        assert!(session.last_active > first_active);
    }

    #[test]
    fn test_session_is_expired() {
        let mut session = Session::new();
        // Not expired with 24-hour TTL
        assert!(!session.is_expired(DEFAULT_SESSION_TTL));
        // Force expiration with a zero TTL
        session.last_active = Instant::now() - Duration::from_secs(1);
        assert!(session.is_expired(Duration::from_millis(500)));
    }

    #[test]
    fn test_session_store_with_ttl() {
        let ttl = Duration::from_secs(3600);
        let store = SessionStore::with_ttl(ttl);
        assert_eq!(store.ttl(), ttl);
    }

    #[test]
    fn test_cleanup_expired() {
        let store = SessionStore::with_ttl(Duration::from_millis(50));
        let _id1 = store.create_session();
        let _id2 = store.create_session();
        assert_eq!(store.session_count(), 2);

        // Wait for sessions to expire
        std::thread::sleep(Duration::from_millis(100));

        let removed = store.cleanup_expired();
        assert_eq!(removed, 2);
        assert_eq!(store.session_count(), 0);
    }

    #[test]
    fn test_expired_session_returns_none() {
        let store = SessionStore::with_ttl(Duration::from_millis(50));
        let id = store.create_session();

        // Session is fresh — should be accessible
        assert!(store.get_session(&id).is_some());

        // Wait for it to expire
        std::thread::sleep(Duration::from_millis(100));

        // Expired — should return None and be cleaned up
        assert!(store.get_session(&id).is_none());
        assert_eq!(store.session_count(), 0);
    }

    #[test]
    fn test_get_session_mut_touches() {
        let store = SessionStore::with_ttl(Duration::from_millis(200));
        let id = store.create_session();

        // Access the session to refresh it
        std::thread::sleep(Duration::from_millis(100));
        {
            let _session = store.get_session_mut(&id).unwrap();
            // touch() is called automatically
        }

        // Wait a bit more — session should still be alive because we touched it
        std::thread::sleep(Duration::from_millis(120));
        assert!(store.get_session(&id).is_some());
    }
}
