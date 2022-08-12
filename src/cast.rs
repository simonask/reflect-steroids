use bevy_reflect::{Reflect, TypeRegistry};

use crate::TypeRegistryExt;

use crate::{DowncastReflect, DynamicCaster, DynamicTrait, DynamicTraitExt, TypeError};

/// Trait object casting interface.
pub trait Cast<P: ?Sized>: Sized {
    /// The target pointer type for this cast.
    type Target;

    /// Try casting `self` using type information from `registry`.
    fn try_cast_with_registry(self, registry: &TypeRegistry) -> Result<Self::Target, TypeError>;

    /// Try casting `self` using the current global type registry (see
    /// [`TypeRegistryExt::set_current()`]).
    fn try_cast(self) -> Result<Self::Target, TypeError> {
        TypeRegistry::with_current(|registry| self.try_cast_with_registry(registry))
    }
}

impl<'a, T, P> Cast<P> for &'a T
where
    P: DynamicTrait + ?Sized,
    T: Reflect + ?Sized,
{
    type Target = &'a P;
    fn try_cast_with_registry(self, registry: &TypeRegistry) -> Result<&'a P, TypeError> {
        CastRef::try_cast_ref_with_registry(self, registry)
    }
}

impl<'a, T, P> Cast<P> for &'a mut T
where
    P: DynamicTrait + ?Sized,
    T: Reflect + ?Sized,
{
    type Target = &'a mut P;
    fn try_cast_with_registry(self, registry: &TypeRegistry) -> Result<&'a mut P, TypeError> {
        CastMut::try_cast_mut_with_registry(self, registry)
    }
}

impl<'a, T, P> Cast<P> for Box<T>
where
    P: DynamicTrait + ?Sized,
    T: DowncastReflect + ?Sized,
{
    type Target = Box<P>;
    fn try_cast_with_registry(self, registry: &TypeRegistry) -> Result<Box<P>, TypeError> {
        CastBox::try_cast_box_with_registry(self, registry).map_err(|(_, err)| err)
    }
}

/// Box casting interface.
pub trait CastBox<T: ?Sized>: Sized {
    /// Try casting `self` using type information from `registry`.
    fn try_cast_box_with_registry<P: DynamicTrait + ?Sized>(
        self,
        registry: &TypeRegistry,
    ) -> Result<Box<P>, (Self, TypeError)>;

    /// Try casting `self` using the current global type registry (see
    /// [`TypeRegistryExt::set_current()`]).
    fn try_cast_box<P: DynamicTrait + ?Sized>(self) -> Result<Box<P>, (Self, TypeError)> {
        TypeRegistry::with_current(|registry| self.try_cast_box_with_registry(registry))
    }
}

/// Reference casting interface.
pub trait CastRef<'a, T: ?Sized>: Sized + 'a {
    /// Try casting `self` using type information from `registry`.
    fn try_cast_ref_with_registry<P: DynamicTrait + ?Sized>(
        self,
        registry: &TypeRegistry,
    ) -> Result<&'a P, TypeError>;

    /// Try casting `self` using the current global type registry (see
    /// [`TypeRegistryExt::set_current()`]).
    fn try_cast_ref<P: DynamicTrait + ?Sized>(self) -> Result<&'a P, TypeError> {
        TypeRegistry::with_current(|registry| self.try_cast_ref_with_registry(registry))
    }
}

/// Mutable reference casting interface.
pub trait CastMut<'a, T: ?Sized>: Sized + 'a {
    /// Try casting `self` using type information from `registry`.
    fn try_cast_mut_with_registry<P: DynamicTrait + ?Sized>(
        self,
        registry: &TypeRegistry,
    ) -> Result<&'a mut P, TypeError>;

    /// Try casting `self` using the current global type registry (see
    /// [`TypeRegistryExt::set_current()`]).
    fn try_cast_mut<P: DynamicTrait + ?Sized>(self) -> Result<&'a mut P, TypeError> {
        TypeRegistry::with_current(|registry| self.try_cast_mut_with_registry(registry))
    }
}

impl<T> CastBox<T> for Box<T>
where
    T: DowncastReflect + ?Sized,
{
    fn try_cast_box_with_registry<P: DynamicTrait + ?Sized>(
        self,
        registry: &TypeRegistry,
    ) -> Result<Box<P>, (Self, TypeError)> {
        // try casting through a ref to avoid having to cast back on failure
        match self.try_cast_ref_with_registry::<P>(registry) {
            Ok(_) => (),
            Err(err) => return Err((self, err)),
        };

        // cast will succeed
        let this = self.downcast_into_reflect();
        let metadata = P::get_type_data_for_object(&*this, registry).unwrap();
        Ok(metadata.from_reflect(this))
    }
}

impl<'a, T> CastRef<'a, T> for &'a T
where
    T: Reflect + ?Sized,
{
    fn try_cast_ref_with_registry<P: DynamicTrait + ?Sized>(
        self,
        registry: &TypeRegistry,
    ) -> Result<&'a P, TypeError> {
        let this = self.as_reflect();
        let metadata = P::get_type_data_for_object(this, registry)?;
        Ok(metadata.from_reflect_ref(this))
    }
}

impl<'a, T> CastRef<'a, T> for &'a mut T
where
    T: Reflect + ?Sized,
{
    fn try_cast_ref_with_registry<P: DynamicTrait + ?Sized>(
        self,
        registry: &TypeRegistry,
    ) -> Result<&'a P, TypeError> {
        let this = self.as_reflect();
        let metadata = P::get_type_data_for_object(this, registry)?;
        Ok(metadata.from_reflect_ref(this))
    }
}

impl<'a, T> CastMut<'a, T> for &'a mut T
where
    T: Reflect + ?Sized,
{
    fn try_cast_mut_with_registry<P: DynamicTrait + ?Sized>(
        self,
        registry: &TypeRegistry,
    ) -> Result<&'a mut P, TypeError> {
        let this = self.as_reflect_mut();
        let metadata = P::get_type_data_for_object(this, registry)?;
        Ok(metadata.from_reflect_mut(this))
    }
}
