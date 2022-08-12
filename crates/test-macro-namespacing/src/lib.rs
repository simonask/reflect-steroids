#[bevy_reflect::reflect_trait]
pub trait TestTrait: reflect_steroids::DowncastReflect {}

reflect_steroids::impl_dynamic_trait!(TestTrait, ReflectTestTrait);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
