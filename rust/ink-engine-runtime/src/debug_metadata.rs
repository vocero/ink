use std::cmp;

pub struct DebugMetadata {
    file_name: String,
    source_name: String,
    start_line_number: i32,
    start_character_number: i32,
    end_line_number: i32,
    end_character_number: i32,
}

impl DebugMetadata {
    pub fn new(
        file_name: String,
        source_name: String,
        start_line_number: i32,
        start_character_number: i32,
        end_line_number: i32,
        end_character_number: i32,
    ) -> Self {
        Self {
            file_name,
            source_name,
            start_line_number,
            start_character_number,
            end_line_number,
            end_character_number,
        }
    }

    pub fn merge(&self, dm: &DebugMetadata) -> DebugMetadata {
        let mut ret_val = DebugMetadata::new(
            self.file_name,
            self.source_name,
            self.start_character_number,
            self.start_line_number,
            self.end_line_number,
            self.end_character_number,
        );

        if ret_val.start_line_number > dm.start_line_number {
            ret_val.start_line_number = dm.start_line_number;
            ret_val.start_character_number = dm.start_character_number;
        } else if ret_val.start_line_number == dm.start_line_number
            && ret_val.start_character_number > dm.start_character_number
        {
            ret_val.start_character_number = dm.start_character_number;
        }

        if ret_val.end_line_number < dm.end_line_number {
            ret_val.end_line_number = dm.end_line_number;
            ret_val.end_character_number = dm.end_character_number;
        } else if ret_val.end_line_number == dm.end_line_number
            && ret_val.end_character_number < dm.end_character_number
        {
            ret_val.end_character_number = dm.end_character_number;
        }

        return ret_val;
    }
}
