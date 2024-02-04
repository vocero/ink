use crate::debug_metadata::DebugMetadata;

pub struct Object<'a> {
    parent: &'a Object<'a>,
    debug_metadata: DebugMetadata,
}
