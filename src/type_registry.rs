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

    /// Include types in the type registry that have been mentioned by the
    /// [`enable_global_type_registration`](crate::enable_global_type_registration)
    /// macro.
    #[cfg(feature = "inventory")]
    fn register_global_types(&mut self);
}

#[cfg(feature = "inventory")]
inventory::collect!(crate::global_registration::RegisterFn);

impl TypeRegistryExt for TypeRegistry {
    fn set_current<F: FnOnce() -> R, R>(&self, f: F) -> R {
        CURRENT_TYPE_REGISTRY.set(self, f)
    }

    #[cfg(feature = "inventory")]
    fn register_global_types(&mut self) {
        for register_fn in inventory::iter::<crate::global_registration::RegisterFn> {
            (register_fn.0)(self);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use bevy_reflect::TypeRegistry;

    use crate::prelude::*;

    #[derive(Reflect)]
    struct TestGlobal;
    enable_global_type_registration!(TestGlobal);

    #[test]
    fn global_registration() {
        let mut registry = TypeRegistry::new();
        registry.register_global_types();
        registry
            .get(TypeId::of::<TestGlobal>())
            .expect("not registered");
    }
}
