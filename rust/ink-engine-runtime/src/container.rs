use crate::{named_content::NamedContent, object::Object};

pub struct Container {
    name: String,
    //content: Vec<Object<'a>>,
}

impl NamedContent for Container {
    fn get_name(&self) -> &String { &self.name }

    fn has_valid_name(&self) -> bool { self.name.len() > 0 }
}
