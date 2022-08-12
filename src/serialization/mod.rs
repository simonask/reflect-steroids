//! Type-tagged serialization/deserialization utilities.
//!
//! This module may be used in a `#[serde(with = "...")]` field attribute when
//! the type of the field is `Box<dyn Reflect>`.

mod de;
mod ser;
mod value;

pub use de::*;
pub use ser::*;

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::reflect::TypeRegistry;

    #[reflect_trait]
    trait MyTrait: DowncastReflect {}
    impl_dynamic_trait!(MyTrait, ReflectMyTrait);

    #[derive(Reflect, serde::Serialize, serde::Deserialize)]
    #[reflect(MyTrait, Serialize, Deserialize)]
    struct Foo {
        num: i32,
    }
    impl MyTrait for Foo {}

    #[derive(Reflect, serde::Serialize, serde::Deserialize)]
    #[reflect(MyTrait, Serialize, Deserialize)]
    struct Bar(i32, i32);
    impl MyTrait for Bar {}

    impl MyTrait for i32 {}

    #[test]
    fn serialize() {
        let trait_object: Box<dyn MyTrait> = Box::new(Foo { num: 123 });
        let mut registry = TypeRegistry::default();
        registry.register::<Foo>();
        registry.set_current(|| {
            let json = serde_json::to_string(&*trait_object).unwrap();
            assert_eq!(json, r#"{"type":"Foo","num":123}"#);

            let deserialized: Box<dyn MyTrait> = serde_json::from_str(&json).unwrap();
            let foo = deserialized.downcast_ref::<Foo>().unwrap();
            assert_eq!(foo.num, 123);
        });
    }

    #[test]
    fn serialize_tuple() {
        let trait_object: &dyn MyTrait = &Bar(123, 456);
        let mut registry = TypeRegistry::default();
        registry.register::<Bar>();
        registry.set_current(|| {
            let json = serde_json::to_string(trait_object).unwrap();
            assert_eq!(json, r#"{"type":"Bar","value":[123,456]}"#);

            let deserialized: Box<dyn MyTrait> = serde_json::from_str(&json).unwrap();
            let bar = deserialized.downcast_ref::<Bar>().unwrap();
            assert_eq!(bar.0, 123);
            assert_eq!(bar.1, 456);
        });
    }

    #[test]
    fn serialize_primitive() {
        let trait_object: &dyn MyTrait = &123i32;
        let mut registry = TypeRegistry::default();
        registry.register_type_data::<i32, ReflectMyTrait>();
        registry.set_current(|| {
            let json = serde_json::to_string(trait_object).unwrap();
            assert_eq!(json, r#"{"type":"i32","value":123}"#);

            let deserialized: Box<dyn MyTrait> = serde_json::from_str(&json).unwrap();
            let num = deserialized.downcast_ref::<i32>().unwrap();
            assert_eq!(*num, 123);
        });
    }

    #[derive(Reflect, serde::Serialize, serde::Deserialize)]
    #[reflect(MyTrait, Serialize, Deserialize)]
    pub struct Nested {
        a: Box<dyn MyTrait>,
        b: Box<dyn MyTrait>,
    }
    impl MyTrait for Nested {}

    #[test]
    fn serialize_nested() {
        let nested: Box<dyn MyTrait> = Box::new(Nested {
            a: Box::new(123i32),
            b: Box::new(Nested {
                a: Box::new(Foo { num: 456 }),
                b: Box::new(Bar(789, 999)),
            }),
        });

        let mut registry = TypeRegistry::default();
        registry.register_type_data::<i32, ReflectMyTrait>();
        registry.register::<Foo>();
        registry.register::<Bar>();
        registry.register::<Nested>();
        registry.set_current(|| {
            let json = serde_json::to_string(&nested).unwrap();
            assert_eq!(json, r#"{"type":"Nested","a":{"type":"i32","value":123},"b":{"type":"Nested","a":{"type":"Foo","num":456},"b":{"type":"Bar","value":[789,999]}}}"#);
            let deserialized: Box<dyn MyTrait> = serde_json::from_str(&json).unwrap();

            let nested = deserialized.downcast_ref::<Nested>().unwrap();
            let a1 = nested.a.downcast_ref::<i32>().unwrap();
            assert_eq!(*a1, 123);
            let b1 = nested.b.downcast_ref::<Nested>().unwrap();
            let a2 = b1.a.downcast_ref::<Foo>().unwrap();
            assert_eq!(a2.num, 456);
            let b2 = b1.b.downcast_ref::<Bar>().unwrap();
            assert_eq!(b2.0, 789);
            assert_eq!(b2.1, 999);
        });
    }
}
