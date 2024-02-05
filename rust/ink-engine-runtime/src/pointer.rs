use crate::{container::Container, object::Object, path::Path};

pub struct Pointer<'a> {
    container: Option<&'a Container>,
    index: i32,
}

impl<'a> Pointer<'a> {
    pub fn new(container: Option<&'a Container>, index: i32) -> Self { Self { container, index } }

    pub fn is_null(&self) -> bool { self.container.is_none() }

    // pub fn resolve(&self) -> Option<Box<Object>> {
    //     match self.container {
    //         Some(ref container)
    //             if self.index >= 0 && (self.index as usize) < container.content.len() =>
    //         {
    //             container.content.get(self.index as usize).cloned()
    //         }
    //         _ => None,
    //     }
    // }

    // pub fn path(&self) -> Option<Path> {
    //     if self.is_null() {
    //         None
    //     } else if let Some(ref container) = self.container {
    //         if self.index >= 0 {
    //             Some(
    //                 container
    //                     .path()
    //                     .path_by_appending_component(PathComponent::Index(self.index)),
    //             )
    //         } else {
    //             // Assuming there is a different logic for negative index which is not provided in the input code.
    //             None
    //         }
    //     } else {
    //         None
    //     }
    // }
}
