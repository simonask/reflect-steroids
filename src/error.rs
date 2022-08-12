use std::borrow::Cow;

/// Type casting errors.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TypeError {
    /// The type was not part of the [`bevy_reflect::TypeRegistry`].
    #[error("unregistered type: {0}")]
    UnregisteredType(Cow<'static, str>),
    /// The type was either not registered with the
    /// [`bevy_reflect::TypeRegistry`], or the type's short name was ambiguous.
    /// See [`bevy_reflect::TypeRegistry::get_with_short_name()`].
    #[error(r#"unknown short type name "{0}" - it may be unregistered, or ambiguous"#)]
    UnregisteredShortName(Cow<'static, str>),
    /// The trait was not registered for the type, i.e., `#[reflect(Trait)]` was
    /// missing from the struct.
    #[error("#[reflect({1})] is missing from '{0}'")]
    UnregisteredTrait(Cow<'static, str>, &'static str),
}
