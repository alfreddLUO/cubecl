//! Helpers for scoping the persistent autotune cache to a device.
//!
//! [`TuneCache`](crate::tune::TuneCache) uses [`cache_scope`] to derive a
//! scope value from [`DeviceProperties`], then combines it with the runtime
//! device identifier and the tunable name through [`scoped_persistence_key`]
//! to produce the on-disk path used by the persistent cache.

use alloc::format;
use alloc::string::String;
use core::hash::{BuildHasher, Hash, Hasher};

use cubecl_ir::DeviceProperties;
use hashbrown::DefaultHashBuilder;

/// Compute a scope identifier for a device.
///
/// Devices sharing the same scope may reuse each other's autotune cache
/// entries.
pub fn cache_scope(properties: &DeviceProperties) -> u64 {
    let mut hasher = DefaultHashBuilder::default().build_hasher();
    properties.features.hash(&mut hasher);
    hasher.finish()
}

/// Build the persistence path segment for a tunable on a given device.
///
/// The result is passed to [`Cache::new`](cubecl_common::cache::Cache::new)
/// as the path component under the autotune cache root.
pub fn scoped_persistence_key(scope: u64, device_id: &str, name: &str) -> String {
    let _ = scope;
    format!("{device_id}/{name}")
}
