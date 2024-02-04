use crate::story_state::StoryState;

pub struct Story {
    state: StoryState,
}

// Version numbers are for engine itself and story file, rather
// than the story state save format
//  -- old engine, new format: always fail
//  -- new engine, old format: possibly cope, based on this number
// When incrementing the version number above, the question you
// should ask yourself is:
//  -- Will the engine be able to load an old story file from
//     before I made these changes to the engine?
//     If possible, you should support it, though it's not as
//     critical as loading old save games, since it's an
//     in-development problem only.

/// The minimum legacy version of ink that can be loaded by the current version of the code.
const INK_VERSION_MINIMUM_COMPATIBLE: i32 = 18;

impl Story {
    /// The current version of the ink story file format.
    const INK_VERSION_CURRENT: i32 = 21;

    /// Construct a Story object using a JSON string compiled through inklecate.
    pub fn new_from_json(json: String) -> Self {
        Story {
            state: StoryState::new(),
        }
    }
}
