use std::{rc::Rc, sync::Arc};

use bevy_reflect::Reflect;

/// Similar to the `Downcast` trait from [`::downcast-rs`], but using the
/// [`Reflect`] trait instead of `std::any::Any`.
pub trait DowncastReflect: Reflect {
    /// Convert `Box<Self>` to [`Box<dyn Reflect>`](bevy_reflect::Reflect).
    fn downcast_into_reflect(self: Box<Self>) -> Box<dyn Reflect>;
    /// Convert `Rc<Self>` to [`Rc<dyn Reflect>`](bevy_reflect::Reflect).
    fn downcast_into_reflect_rc(self: Rc<Self>) -> Rc<dyn Reflect>;
    /// Convert `Arc<Self>` to [`Arc<dyn Reflect>`](bevy_reflect::Reflect).
    fn downcast_into_reflect_arc(self: Arc<Self>) -> Arc<dyn Reflect>;
}

impl DowncastReflect for dyn Reflect {
    fn downcast_into_reflect(self: Box<Self>) -> Box<dyn Reflect> {
        self
    }

    fn downcast_into_reflect_rc(self: Rc<Self>) -> Rc<dyn Reflect> {
        self
    }

    fn downcast_into_reflect_arc(self: Arc<Self>) -> Arc<dyn Reflect> {
        self
    }
}

impl<T: Reflect> DowncastReflect for T {
    fn downcast_into_reflect(self: Box<Self>) -> Box<dyn Reflect> {
        self
    }

    fn downcast_into_reflect_rc(self: Rc<Self>) -> Rc<dyn Reflect> {
        self
    }

    fn downcast_into_reflect_arc(self: Arc<Self>) -> Arc<dyn Reflect> {
        self
    }
}
