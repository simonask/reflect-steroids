use bevy_reflect::TypeRegistry;

scoped_tls::scoped_thread_local!(
    static CURRENT_TYPE_REGISTRY: TypeRegistry
);

/// Extension methods for [`TypeRegistry`](bevy_reflect::TypeRegistry).
pub trait TypeRegistryExt {
    /// True if a registry has been set for the current thread with
    /// [`TypeRegistryExt::set_current`].
    fn has_current() -> bool {
        CURRENT_TYPE_REGISTRY.is_set()
    }

    /// Get the current thread's [`TypeRegistry`].
    fn with_current<F: FnOnce(&TypeRegistry) -> R, R>(f: F) -> R {
        CURRENT_TYPE_REGISTRY.with(f)
    }

    /// Set the current thread's [`TypeRegistry`].
    ///
    /// Upon return, the previously set [`TypeRegistry`] will become current
    /// again.
    fn set_current<F: FnOnce() -> R, R>(&self, f: F) -> R;
}

impl TypeRegistryExt for TypeRegistry {
    fn set_current<F: FnOnce() -> R, R>(&self, f: F) -> R {
        CURRENT_TYPE_REGISTRY.set(self, f)
    }
}
