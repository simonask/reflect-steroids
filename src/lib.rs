#![doc = include_str!("../README.md")]
#![deny(missing_docs, clippy::useless_conversion, clippy::useless_asref)]
#![warn(clippy::pedantic)]

mod cast;
mod downcast;
mod dynamic_trait;
mod error;
pub mod serialization;
mod type_registry;

pub use cast::*;
pub use downcast::*;
pub use dynamic_trait::*;
pub use error::*;
pub use type_registry::*;

#[doc(no_inline)]
pub use bevy_reflect as reflect;

#[doc(no_inline, hidden)]
pub use serde;

/// Prelude
pub mod prelude {
    #[doc(no_inline)]
    pub use super::{
        impl_dynamic_trait, reflect::prelude::*, Cast as _, CastBox as _, CastMut as _,
        CastRef as _, DowncastReflect, DynamicTraitExt as _, TypeRegistryExt as _,
    };
}

/// Implement [`DynamicTrait`] for a trait object.
///
/// This enables dynamic trait casting for `dyn MyTrait`, via
/// [`bevy_reflect::TypeRegistry`].
///
/// The second argument is the "hidden" [`TypeData`](bevy_reflect::TypeData) of
/// the trait, generated by [`#[reflect_trait]`](bevy_reflect::reflect_trait).
/// The name of this type is "ReflectXYZ", where "XYZ" is the name of the trait.
///
/// This macros implements the following:
///
/// - [`DynamicTrait`] for `dyn MyTrait`.
/// - [`DynamicCaster`] for `ReflectMyTrait`.
/// - [`Reflect`](bevy_reflect::Reflect) for `Box<dyn MyTrait>`, which forwards
///   all reflection to the concrete type (meaning it can be reflected upon as
///   if it was `Box<dyn Reflect>`).
/// - [`std::fmt::Debug`] for `dyn MyTrait` (forwarding to
///   [`Reflect::debug()`](bevy_reflect::Reflect::debug)).
/// - [`Serialize`](serde::Serialize) for `dyn MyTrait` (see [serialization]).
/// - [`Deserialize`](serde::Deserialize) for `Box<dyn MyTrait>` (see
///   [serialization]).
/// - A downcasting interface for `dyn MyTrait`, similar to `dyn Reflect` and
///   `dyn Any`.
///
/// ## Usage
///
/// ```rust
/// # use reflect_steroids::prelude::*;
/// #[reflect_trait]
/// trait MyTrait: DowncastReflect {}
///
/// impl_dynamic_trait!(MyTrait, ReflectMyTrait);
///
/// #[derive(Reflect)]
/// #[reflect(MyTrait)]
/// struct Foo;
///
/// impl MyTrait for Foo {}
///
/// let foo = &Foo;
/// let foo_as_mytrait: &dyn MyTrait = foo;
/// let foo_as_reflect = foo_as_mytrait.as_reflect();
/// assert_eq!(foo_as_reflect.type_name(), std::any::type_name::<Foo>());
/// ```
#[macro_export]
macro_rules! impl_dynamic_trait {
    ($trait_name:ident, $type_data_name:ident) => {
        impl $crate::DynamicTrait for dyn $trait_name {
            type TypeData = $type_data_name;

            fn reflect_name() -> &'static str {
                stringify!($trait_name)
            }
        }

        impl $crate::DynamicCaster<dyn $trait_name> for $type_data_name {
            fn from_reflect(&self, this: Box<dyn Reflect>) -> Box<dyn $trait_name> {
                self.get_boxed(this).unwrap()
            }

            fn from_reflect_ref<'a>(&self, this: &'a dyn Reflect) -> &'a dyn $trait_name {
                self.get(this).unwrap()
            }

            fn from_reflect_mut<'a>(&self, this: &'a mut dyn Reflect) -> &'a mut dyn $trait_name {
                self.get_mut(this).unwrap()
            }
        }

        #[allow(dead_code)]
        impl dyn $trait_name {
            pub fn is<T: $trait_name>(&self) -> bool {
                self.as_reflect().is::<T>()
            }

            pub fn downcast<T: $trait_name>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
                if self.is::<T>() {
                    Ok(<dyn $crate::reflect::Reflect>::downcast(
                        $crate::DowncastReflect::downcast_into_reflect(self),
                    )
                    .unwrap())
                } else {
                    Err(self)
                }
            }

            pub fn downcast_ref<T: $trait_name>(&self) -> Option<&T> {
                self.as_reflect().downcast_ref()
            }

            pub fn downcast_mut<T: $trait_name>(&mut self) -> Option<&mut T> {
                self.as_reflect_mut().downcast_mut()
            }

            pub fn into_reflect(self: Box<Self>) -> Box<dyn $crate::reflect::Reflect> {
                $crate::DowncastReflect::downcast_into_reflect(self)
            }
        }

        impl $crate::reflect::Reflect for Box<dyn $trait_name> {
            fn type_name(&self) -> &str {
                (**self).as_reflect().type_name()
            }

            fn get_type_info(&self) -> &'static bevy_reflect::TypeInfo {
                (**self).as_reflect().get_type_info()
            }

            fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
                $crate::reflect::Reflect::into_any($crate::DowncastReflect::downcast_into_reflect(
                    self,
                ))
            }

            fn as_any(&self) -> &dyn ::core::any::Any {
                (**self).as_reflect().as_any()
            }

            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                (**self).as_reflect_mut().as_any_mut()
            }

            fn as_reflect(&self) -> &dyn Reflect {
                (**self).as_reflect().as_reflect()
            }

            fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
                (**self).as_reflect_mut().as_reflect_mut()
            }

            fn apply(&mut self, value: &dyn Reflect) {
                (**self).as_reflect_mut().apply(value)
            }

            fn set(&mut self, value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>> {
                (**self).as_reflect_mut().set(value)
            }

            fn reflect_ref(&self) -> bevy_reflect::ReflectRef {
                (**self).as_reflect().reflect_ref()
            }

            fn reflect_mut(&mut self) -> bevy_reflect::ReflectMut {
                (**self).as_reflect_mut().reflect_mut()
            }

            fn clone_value(&self) -> Box<dyn Reflect> {
                (**self).as_reflect().clone_value()
            }

            fn reflect_hash(&self) -> Option<u64> {
                (**self).as_reflect().reflect_hash()
            }

            fn reflect_partial_eq(&self, value: &dyn Reflect) -> Option<bool> {
                (**self).as_reflect().reflect_partial_eq(value)
            }

            fn debug(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                (**self).as_reflect().debug(f)
            }

            fn serializable(&self) -> Option<$crate::reflect::serde::Serializable<'_>> {
                (**self).as_reflect().serializable()
            }
        }

        impl $crate::reflect::FromReflect for Box<dyn $trait_name> {
            fn from_reflect(_: &dyn $crate::reflect::Reflect) -> Option<Self> {
                None
            }
        }

        impl ::core::fmt::Debug for dyn $trait_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                self.as_reflect().debug(f)
            }
        }

        impl ::serde::Serialize for dyn $trait_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                $crate::serialization::serialize(self.as_reflect(), serializer)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for Box<dyn $trait_name> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let box_dyn_reflect = $crate::serialization::deserialize(deserializer)?;
                $crate::Cast::try_cast(box_dyn_reflect).map_err($crate::serde::de::Error::custom)
            }
        }
    };
}
