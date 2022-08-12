use std::{any::Any, ops::Deref};

use bevy_reflect::{Reflect, ReflectRef, ReflectSerialize, TypeRegistry};
use serde::Serialize;

use crate::{TypeError, TypeRegistryExt};

scoped_tls::scoped_thread_local!(
    static CURRENTLY_SERIALIZING_TYPE: String
);

/// Serialize any dynamic trait pointer.
///
/// This populates a `type` field in the serialized data containing the type's
/// "short name". Note that deserialization will fail if the type's short name
/// is ambiguous.
///
/// If the serialized type is a struct, its fields will be serialized alongside
/// the "type" field (flattened). Otherwise, the serialized data will be put in
/// a field with the name "value".
///
/// Serialization requires a current global
/// [`TypeRegistry`](bevy_reflect::TypeRegistry). See
/// [`TypeRegistryExt::set_current`].
///
/// This function may be used in a `#[serde(serialize_with = "...")]` field
/// attribute.
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
/// let trait_object: Box<dyn MyTrait> = Box::new(Foo { num: 123 });
///
/// let mut registry = TypeRegistry::default();
/// registry.register::<Foo>();
/// registry.set_current(|| {
///     let json = serde_json::to_string(&*trait_object).unwrap();
///     assert_eq!(json, r#"{"type":"Foo","num":123}"#);
/// });
/// ```
pub fn serialize<S>(this: &dyn Reflect, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::Error as _;

    let type_name = TypeRegistry::with_current(|registry| {
        let registration = registry.get(this.as_reflect().type_id()).ok_or_else(|| {
            TypeError::UnregisteredType(this.as_reflect().type_name().to_string().into())
        })?;
        let type_name = registration.short_name().to_string();
        Ok(type_name)
    })
    .map_err(|e: TypeError| S::Error::custom(e))?;

    CURRENTLY_SERIALIZING_TYPE.set(&type_name, || {
        match this.reflect_ref() {
            // Serialize flattened.
            ReflectRef::Struct(_) => {
                let serialize = SerializeWithTypeTagFlattened {
                    type_name: CurrentType,
                    value: SerializePointerWithTypeTag {
                        pointer: this.as_reflect(),
                    },
                };
                serialize.serialize(serializer)
            }
            // For all other types, serialize unflattened.
            _ => {
                let serialize = SerializeWithTypeTagUnflattened {
                    type_name: CurrentType,
                    value: SerializePointerWithTypeTag {
                        pointer: this.as_reflect(),
                    },
                };
                serialize.serialize(serializer)
            }
        }
    })
}

#[derive(Serialize)]
#[serde(bound(
    serialize = "Ptr: Deref<Target = dyn Reflect>",
    // deserialize = "SerializePointerWithTypeTag<Ptr>: Deserialize<'de>"
))]
struct SerializeWithTypeTagFlattened<Ptr> {
    #[serde(rename = "type")]
    type_name: CurrentType,
    #[serde(flatten)]
    value: SerializePointerWithTypeTag<Ptr>,
}

#[derive(Serialize)]
#[serde(bound(
    serialize = "Ptr: Deref<Target = dyn Reflect>",
    // deserialize = "SerializePointerWithTypeTag<Ptr>: Deserialize<'de>"
))]
struct SerializeWithTypeTagUnflattened<Ptr> {
    #[serde(rename = "type")]
    type_name: CurrentType,
    value: SerializePointerWithTypeTag<Ptr>,
}

struct SerializePointerWithTypeTag<Ptr> {
    pointer: Ptr,
}

impl<Ptr: Deref<Target = dyn Reflect>> Serialize for SerializePointerWithTypeTag<Ptr> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::Error as _;

        let serialize = TypeRegistry::with_current(|registry| {
            let registration =
                match registry.get(Any::type_id(Reflect::as_any(self.pointer.as_reflect()))) {
                    Some(type_name) => type_name,
                    None => {
                        return Err(TypeError::UnregisteredType(
                            self.pointer.as_reflect().type_name().to_string().into(),
                        ))
                    }
                };
            Ok(registration
                .data::<ReflectSerialize>()
                .ok_or_else(|| {
                    TypeError::UnregisteredTrait(
                        self.pointer.as_reflect().type_name().to_string().into(),
                        "Serialize",
                    )
                })?
                .clone())
        })
        .map_err(S::Error::custom)?;

        let serializable = serialize.get_serializable(&*self.pointer);
        serializable.borrow().serialize(serializer)
    }
}

// impl<'de> Deserialize<'de> for SerializePointerWithTypeTag<Box<dyn Reflect>> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         use serde::de::Error as _;

//         let type_name = CURRENTLY_DESERIALIZING_TYPE
//             .with(|cell| cell.borrow().clone())
//             .expect("internal imbalance - no type being deserialized");

//         let deserialize = TypeRegistry::with_current(|registry| {
//             let registration = match registry.get_with_short_name(&type_name) {
//                 Some(registration) => registration,
//                 None => return Err(TypeError::UnregisteredTypeName(type_name.into())),
//             };
//             Ok(registration
//                 .data::<ReflectDeserialize>()
//                 .ok_or_else(|| TypeError::UnregisteredTrait(type_name.into(), "Deserialize"))?
//                 .clone())
//         })
//         .map_err(|e: TypeError| D::Error::custom(e))?;

//         Ok(SerializePointerWithTypeTag {
//             pointer: deserialize.deserialize(deserializer)?,
//         })
//     }
// }

/// When (de)serializing, this registers the type name in the current
/// (de)serialization context, such that when `SerializePointerWithTypeTag<P>`
/// is (de)serialized, the concrete type name is known.
struct CurrentType;

impl Serialize for CurrentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        CURRENTLY_SERIALIZING_TYPE.with(|name| name.serialize(serializer))
    }
}
