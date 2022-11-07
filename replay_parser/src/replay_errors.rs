//! This module contains code that deals with fixing bugs withing the actual replay format

use std::borrow::Cow;

/// Some replays contains bug with the json format. This function tries to fix some of them
pub fn fix_json_bugs(input: Cow<str>) -> String {
    remove_racing_time_field(input)
}

/// replays from version `1.16.0.0` contains a field called `racingFinishTime` that has a syntax error.
/// This fixes it by removing that field altogether
fn remove_racing_time_field(input: Cow<str>) -> String {
    input.replace("\"racingFinishTime\": Infinity,", "")
}
