pub mod data;
pub(crate) mod deserializer;
mod serializer;

#[derive(Default)]
pub struct Save {
    pub resources: data::Resources,
}
impl Save {
    
}
