pub mod data;
pub(crate) mod deserializer;
mod serializer;

#[derive(Default)]
pub struct Save {
    pub resources: data::Resources,
    pub classes: data::Classes,
}
impl Save {
    
}
