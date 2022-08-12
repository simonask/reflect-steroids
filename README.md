# Bevy Reflection on Steroids

A series of supplementary extensions to the
[`bevy_reflect`](https://docs.rs/bevy_reflect) crate that powers aspects of the
[Bevy](https://bevyengine.org/) game engine.

`bevy_reflect` provides is a general reflection framework for Rust structs, with
rudimentary support for traits.

## Highlights

- Integrates seamlessly with `bevy_reflect`.
- Includes a facility to perform generic dynamic casts between trait objects,
  patching a hole in the Rust language (similar to C++ `dynamic_cast`). See the
  [`Cast`] trait.
- Includes the equivalent of [`downcast-rs`](https://docs.rs/downcast-rs), but
  using the
  [`Reflect`](https://docs.rs/bevy_reflect/latest/bevy_reflect/trait.Reflect.html)
  trait instead of
  [`Any`](https://doc.rust-lang.org/stable/std/any/trait.Any.html). See
  [`DowncastReflect`].
- Serialization and deserialization of opaque trait objects (similar to
  [`typetag`](https://docs.rs/typetag)).

This crate provides a much more intuitive and user-friendly way of interacting
with reflected types through arbitrary trait objects.

By implementing [`DynamicTrait`] for `dyn MyTrait`, reflection is made available
to all dynamic trait object references of that type. The only requirement is
that the trait has [`DowncastReflect`] as a supertrait, and that a call to the
macro [`impl_dynamic_trait!(MyTrait, ReflectMyTrait)`](impl_dynamic_trait) is
present in the code.
