pub trait NamedContent {
    fn get_name(&self) -> &String;
    fn has_valid_name(&self) -> bool;
}
