//! Compound router keys: `"{engine_id}:{logical_name}"`.

/// Router key for default in-memory backend (`inmemory_mem:default`).
pub const DEFAULT_IN_MEMORY_ROUTER_KEY: &str = "inmemory_mem:default";

/// Build the compound router key used for registration and resolution.
///
/// # Example
///
/// ```
/// use valence_core::router_key;
///
/// assert_eq!(router_key("default", "inmemory_mem"), "inmemory_mem:default");
/// ```
#[must_use]
pub fn router_key(logical_name: &str, engine_id: &str) -> String {
    format!("{engine_id}:{logical_name}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router_key_formats_open_slug() {
        assert_eq!(router_key("billing", "acme_vault"), "acme_vault:billing");
    }
}
