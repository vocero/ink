use crate::choice::Choice;

pub struct StoryState {
    /// The list of Choice objects available at the current point in
    /// the Story. This list will be populated as the Story is stepped
    /// through with the Continue() method. Once canContinue becomes
    /// false, this list will be populated, and is usually
    /// (but not always) on the final Continue() step.
    current_choices: Vec<Choice>,

    /// Gets a list of tags as defined with '#' in source that were seen
    /// during the latest Continue() call.
    current_tags: Vec<String>,

    /// Any errors generated during evaluation of the Story.
    current_errors: Vec<String>,

    /// Any warnings generated during evaluation of the Story.
    current_warnings: Vec<String>,
}

impl StoryState {
    pub fn new() -> Self {
        Self {
            current_choices: Vec::new(),
            current_tags: Vec::new(),
            current_errors: Vec::new(),
            current_warnings: Vec::new(),
        }
    }

    /// The latest line of text to be generated from a Continue() call.
    pub fn get_current_text(self) -> String { todo!() }

    pub fn get_current_choices(self) -> Vec<Choice> { todo!() }
}
