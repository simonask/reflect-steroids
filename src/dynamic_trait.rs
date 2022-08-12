use std::{borrow::Cow, marker::PhantomData};

use bevy_reflect::{Reflect, TypeData, TypeRegistration, TypeRegistry};

use crate::TypeError;

/// Description of an interface. This is a way to associate trait object types
/// with some metadata that lives in the [`TypeRegistry`].
///
/// The metadata describes how to cast a trait object of type `Self::Target`
/// to/from `dyn Reflect`, but may also contain other functions particular to
/// that interface, such as static functions.
///
/// This is a generalization of the metadata generated by
/// [`bevy_reflect::reflect_trait`] that allows us to handle trait objects in
/// generic code. For instance, implementing this trait for `dyn MyTrait`
/// automatically gives you serialization/deserialization of `Box<dyn MyTrait>`.
///
/// Note that `DynamicTrait` is implemented for all `T: Reflect + Sized`,
/// meaning conceptually that all types `T` implement the `T` "interface".
pub trait DynamicTrait: Reflect {
    /// The metadata stored in the [`TypeRegistry`]. This type is generated by
    /// [`bevy_reflect::reflect_trait`].
    type TypeData: TypeData + DynamicCaster<Self>;

    /// The name of the trait. This is for error message friendliness only. When
    /// a trait object cast fails, the error message guides the user to annotate
    /// the struct in question with `#[reflect(MyTrait)]` (when this function
    /// returns "MyTrait").
    fn reflect_name() -> &'static str;

    /// Get the metadata for this trait object.
    ///
    /// The default implementation simply calls
    /// `registration.data::<Self::Metadata>()`, but some implementations are
    /// able to provide the metadata without the [`TypeRegistration`] object. In
    /// particular, this is true for the `DynamicTrait` implementation for [`dyn
    /// Reflect`] as well as the implementation for `T` itself.
    fn get_type_data(registration: &TypeRegistration) -> Option<Cow<'_, Self::TypeData>> {
        Some(Cow::Borrowed(registration.data::<Self::TypeData>()?))
    }
}

/// Convenience methods for all [`DynamicTrait`] implementations.
pub trait DynamicTraitExt: DynamicTrait {
    /// Given a reference to a reflected object and a [`TypeRegistry`], find the
    /// [`Self::TypeData`](DynamicTrait::TypeData) for the object's type and
    /// this [`DynamicTrait`].
    ///
    /// In essence, this returns a suitable [`DynamicCaster`] for casting
    /// `pointer` to this trait object.
    fn get_type_data_for_object<'a>(
        pointer: &dyn Reflect,
        registry: &'a TypeRegistry,
    ) -> Result<Cow<'a, Self::TypeData>, TypeError> {
        let registration = registry
            .get(pointer.as_any().type_id())
            .ok_or_else(|| TypeError::UnregisteredType(pointer.type_name().to_string().into()))?;
        let metadata = Self::get_type_data(registration).ok_or_else(|| {
            TypeError::UnregisteredTrait(
                registration.short_name().to_string().into(),
                Self::reflect_name(),
            )
        })?;
        Ok(metadata)
    }
}

impl<T: DynamicTrait + ?Sized> DynamicTraitExt for T {}

/// Cast a reflected pointer to another trait object.
///
/// This can be implemented for the [`TypeData`](bevy_reflect::TypeData)
/// generated by the [`#[reflect(MyTrait)]`](bevy_reflect::Reflect) attribute,
/// which in that case is named "ReflectMyTrait". It is automatically
/// implemented when using the [`impl_dynamic_trait!(MyTrait,
/// ReflectMyTrait)`](impl_dynamic_trait) macro.
pub trait DynamicCaster<T: ?Sized>: Send + Sync + Clone + 'static {
    /// Cast from box.
    fn from_reflect(&self, this: Box<dyn Reflect>) -> Box<T>;
    /// Cast from reference.
    fn from_reflect_ref<'a>(&self, this: &'a dyn Reflect) -> &'a T;
    /// Cast from mutable reference.
    fn from_reflect_mut<'a>(&self, this: &'a mut dyn Reflect) -> &'a mut T;
}

const _: () = {
    pub struct SelfTrait<T>(PhantomData<T>);

    impl<T: Reflect> SelfTrait<T> {
        pub fn new() -> SelfTrait<T> {
            SelfTrait(PhantomData)
        }
    }

    impl<T: Reflect> DynamicCaster<T> for SelfTrait<T> {
        fn from_reflect(&self, this: Box<dyn Reflect>) -> Box<T> {
            this.into_any().downcast().expect("type mismatch")
        }

        fn from_reflect_ref<'a>(&self, this: &'a dyn Reflect) -> &'a T {
            this.downcast_ref().expect("type mismatch")
        }

        fn from_reflect_mut<'a>(&self, this: &'a mut dyn Reflect) -> &'a mut T {
            this.downcast_mut().expect("type mismatch")
        }
    }

    impl<T: Reflect> Default for SelfTrait<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> Clone for SelfTrait<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T> Copy for SelfTrait<T> {}

    impl<T: Reflect + Sized> DynamicTrait for T {
        type TypeData = SelfTrait<T>;

        fn reflect_name() -> &'static str {
            std::any::type_name::<T>()
        }

        fn get_type_data(_registration: &TypeRegistration) -> Option<Cow<'_, Self::TypeData>> {
            Some(Cow::Owned(SelfTrait::new()))
        }
    }

    #[derive(Clone, Copy)]
    pub struct DynReflectMetadata;

    impl DynamicCaster<dyn Reflect> for DynReflectMetadata {
        fn from_reflect(&self, this: Box<dyn Reflect>) -> Box<dyn Reflect> {
            this
        }

        fn from_reflect_ref<'a>(&self, this: &'a dyn Reflect) -> &'a dyn Reflect {
            this
        }

        fn from_reflect_mut<'a>(&self, this: &'a mut dyn Reflect) -> &'a mut dyn Reflect {
            this
        }
    }

    impl DynamicTrait for dyn Reflect {
        type TypeData = DynReflectMetadata;

        fn reflect_name() -> &'static str {
            "Reflect"
        }

        fn get_type_data(_: &TypeRegistration) -> Option<Cow<'_, Self::TypeData>> {
            Some(Cow::Owned(DynReflectMetadata))
        }
    }
};

#[cfg(test)]
mod tests {
    use bevy_reflect::{reflect_trait, TypeRegistry};

    use crate::{impl_dynamic_trait, Cast, CastMut, CastRef, DowncastReflect, TypeRegistryExt};

    use super::*;

    #[reflect_trait]
    trait Trait1: DowncastReflect {}
    #[reflect_trait]
    trait Trait2: DowncastReflect {}

    impl_dynamic_trait!(Trait1, ReflectTrait1);
    impl_dynamic_trait!(Trait2, ReflectTrait2);

    #[derive(Debug, Reflect)]
    #[reflect(Trait1, Trait2)]
    struct Foo {
        num: i32,
    }

    impl Trait1 for Foo {}
    impl Trait2 for Foo {}

    #[test]
    fn trait_object_reflection() {
        let foo = &Foo { num: 123 };
        let a: &dyn Trait1 = foo;
        let tr: &dyn Reflect = a.as_reflect();
        let fr: &dyn Reflect = foo;
        assert!(tr.is::<Foo>());
        assert!(tr.as_any().is::<Foo>());
        assert_eq!(tr.type_name(), std::any::type_name::<Foo>());
        assert_eq!(tr.as_any() as *const _, fr.as_any() as *const _);
        assert_eq!(tr.as_reflect() as *const _, fr.as_reflect() as *const _);

        let foo = Box::new(Foo { num: 123 });
        let foo: Box<dyn Trait1> = foo;

        let tr: &dyn Reflect = foo.as_reflect();
        assert!(tr.is::<Foo>());
        assert!(tr.as_any().is::<Foo>());
        assert_eq!(tr.type_name(), std::any::type_name::<Foo>());
        match tr.reflect_ref() {
            bevy_reflect::ReflectRef::Struct(reflect_struct) => {
                assert!(reflect_struct
                    .field("num")
                    .expect("expected num field")
                    .is::<i32>());
            }
            _ => panic!("unexpected reflection kind"),
        }

        let foo: Box<dyn Reflect> = foo.into_reflect();
        assert!(foo.is::<Foo>());
        assert!(foo.as_any().is::<Foo>());
        assert_eq!(foo.type_name(), std::any::type_name::<Foo>());
        match foo.reflect_ref() {
            bevy_reflect::ReflectRef::Struct(reflect_struct) => {
                assert!(reflect_struct
                    .field("num")
                    .expect("expected num field")
                    .is::<i32>());
            }
            _ => panic!("unexpected reflection kind"),
        }
    }

    #[test]
    fn basic_casts() {
        let mut registry = TypeRegistry::default();
        registry.register::<Foo>();
        registry.set_current(|| {
            let a: Box<Foo> = Box::new(Foo { num: 123 });
            let b: Box<dyn Trait1> = a;
            assert!(b.is::<Foo>());
            let c: Box<Foo> = Cast::try_cast(b).unwrap();
            let d: Box<dyn Trait2> = Cast::try_cast(c).unwrap();
            assert!(d.is::<Foo>());
            let mut e: Box<dyn Trait1> = Cast::try_cast(d).unwrap();
            assert!(e.is::<Foo>());
            let f: &dyn Trait2 = CastRef::try_cast_ref(&e).unwrap();
            assert!(f.is::<Foo>());
            let g: &mut dyn Trait1 = CastMut::try_cast_mut(&mut e).unwrap();
            assert!(g.is::<Foo>());
        });
    }
}