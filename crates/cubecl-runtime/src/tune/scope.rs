//! Helpers for scoping the persistent autotune cache to a device.
//!
//! [`TuneCache`](crate::tune::TuneCache) uses [`cache_scope`] to derive a
//! scope value from the device's [`HardwareProperties`], then combines it with
//! the runtime device identifier and the tunable name through
//! [`scoped_persistence_key`] to produce the on-disk path used by the
//! persistent cache.

use alloc::format;
use alloc::string::String;
use core::hash::{BuildHasher, Hash, Hasher};

use ahash::RandomState;
use cubecl_ir::HardwareProperties;

/// Fixed seeds so the scope is stable across processes (the persistent cache is
/// on disk, so a randomized hash would never match a previous run).
const SCOPE_SEEDS: (u64, u64, u64, u64) = (
    0x0123_4567_89ab_cdef,
    0xfedc_ba98_7654_3210,
    0xdead_beef_cafe_babe,
    0x1337_c0de_f00d_face,
);

/// Compute a scope identifier for a device from its hardware properties.
///
/// Devices sharing the same scope may reuse each other's autotune cache
/// entries. The scope is derived from the device's binding capacity, which
/// bounds the resources a tuned kernel can address on the device.
pub fn cache_scope(hardware: &HardwareProperties) -> u64 {
    let mut hasher =
        RandomState::with_seeds(SCOPE_SEEDS.0, SCOPE_SEEDS.1, SCOPE_SEEDS.2, SCOPE_SEEDS.3)
            .build_hasher();
    hardware.max_bindings.hash(&mut hasher);
    hasher.finish()
}

/// Build the persistence path segment for a tunable on a given device.
///
/// The result is passed to [`Cache::new`](cubecl_common::cache::Cache::new)
/// as the path component under the autotune cache root.
pub fn scoped_persistence_key(scope: u64, device_id: &str, name: &str) -> String {
    format!("{scope:016x}/{device_id}/{name}")
}
