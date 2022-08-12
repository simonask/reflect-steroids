use std::{borrow::Cow, cell::RefCell, collections::HashMap};

use bevy_reflect::{Reflect, ReflectDeserialize, TypeInfo, TypeRegistry};
use serde::{
    de::{value::MapDeserializer, IntoDeserializer},
    Deserialize,
};

use crate::{TypeError, TypeRegistryExt};

use super::value::Value;

scoped_tls::scoped_thread_local!(
    static CURRENTLY_DESERIALIZING_TYPE: RefCell<Option<String>>
);

/// Deserialize any dynamic trait pointer.
///
/// The data is expected to contain a field named `type`, which indicates the
/// short name of the
///
/// Deserialization requires a current global
/// [`TypeRegistry`](bevy_reflect::TypeRegistry). See
/// [`TypeRegistryExt::set_current`].
///
/// This function may be used in a `#[serde(deserialize_with = "...")]`
/// field attribute.
///
/// ## Example
/// ```rust
/// # use reflect_steroids::{prelude::*, reflect::TypeRegistry};
/// #[reflect_trait]
/// trait MyTrait: DowncastReflect {}
/// impl_dynamic_trait!(MyTrait, ReflectMyTrait);
///
/// #[derive(Reflect, serde::Serialize, serde::Deserialize)]
/// #[reflect(MyTrait, Serialize, Deserialize)]
/// struct Foo { num: i32 }
///
/// impl MyTrait for Foo {}
///
///
/// let mut registry = TypeRegistry::default();
/// registry.register::<Foo>();
///
/// let json = r#"{"type":"Foo","num":123}"#;
///
/// registry.set_current(|| {
///     let trait_object: Box<dyn MyTrait> = serde_json::from_str(json).unwrap();
///     assert!(trait_object.is::<Foo>());
///     let foo = trait_object.downcast_ref::<Foo>().unwrap();
///     assert_eq!(foo.num, 123);
/// });
/// ```
pub fn deserialize<'de, D>(deserializer: D) -> Result<Box<dyn Reflect>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error as _;

    let DeserializeWithTypeTag {
        type_name,
        mut value_map,
    } = DeserializeWithTypeTag::deserialize(deserializer)?;

    let (type_info, deserialize) = TypeRegistry::with_current(|registry| {
        let registration = match registry.get_with_short_name(&type_name) {
            Some(registration) => registration,
            None => {
                return Err(TypeError::UnregisteredShortName(
                    type_name.into_owned().into(),
                ))
            }
        };

        let deserialize = match registration.data::<ReflectDeserialize>() {
            Some(deserialize) => deserialize,
            None => {
                return Err(TypeError::UnregisteredTrait(
                    registration.type_name().to_string().into(),
                    "Deserialize",
                ));
            }
        };

        Ok((registration.type_info(), deserialize.clone()))
    })
    .map_err(D::Error::custom)?;

    // If the type is a struct, deserialize it with fields from `value_map`.
    // Otherwise, expect the field `value` and deserialize that.

    match type_info {
        TypeInfo::Struct(_) => {
            let fields: MapDeserializer<_, D::Error> = value_map.into_deserializer();
            deserialize.deserialize(fields)
        }
        _ => {
            let value = if let Some(value) = value_map.remove("value") {
                value
            } else {
                return Err(D::Error::custom(
                    "expected field `value` for type-erased deserialization of non-struct type",
                ));
            };

            let value_deserializer = value.into_deserializer();
            deserialize.deserialize(value_deserializer)
        }
    }
}

#[derive(Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
struct DeserializeWithTypeTag<'a> {
    #[serde(rename = "type")]
    type_name: Cow<'a, str>,
    #[serde(flatten)]
    value_map: HashMap<Cow<'a, str>, Value<'a>>,
}
