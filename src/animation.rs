use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use std::rc::Rc;

use crate::{
    CURSOR_CLASS,
    repeat::Repeat,
    sequence::{SequenceElement, StringSplitter},
    speed::Speed,
};

#[derive(Clone, Copy)]
pub(crate) struct AnimationConfig {
    pub(crate) repeat: Repeat,
    pub(crate) speed: u64,
    pub(crate) deletion_speed: u64,
    pub(crate) omit_deletion_animation: bool,
}

pub(crate) async fn run_animation(
    sequence: &[SequenceElement],
    splitter: &StringSplitter,
    config: AnimationConfig,
    mut current_text: String,
    displayed: &mut Signal<String>,
) {
    match config.repeat {
        Repeat::Count(count) => {
            for _ in 0..=count {
                current_text = run_sequence_once(
                    sequence,
                    splitter,
                    config.speed,
                    config.deletion_speed,
                    config.omit_deletion_animation,
                    current_text,
                    displayed,
                )
                .await;
            }
        }
        Repeat::Infinite => loop {
            current_text = run_sequence_once(
                sequence,
                splitter,
                config.speed,
                config.deletion_speed,
                config.omit_deletion_animation,
                current_text,
                displayed,
            )
            .await;
        },
    }
}

async fn run_sequence_once(
    sequence: &[SequenceElement],
    splitter: &StringSplitter,
    speed: u64,
    deletion_speed: u64,
    omit_deletion_animation: bool,
    mut current_text: String,
    displayed: &mut Signal<String>,
) -> String {
    for item in sequence {
        match item {
            SequenceElement::Text(next_text) => {
                let edits = edits_for_transition(&current_text, next_text, splitter);
                perform_edits(
                    displayed,
                    &edits,
                    speed,
                    deletion_speed,
                    omit_deletion_animation,
                    &mut current_text,
                )
                .await;
                current_text = next_text.clone();
            }
            SequenceElement::Delay(ms) => wait(*ms).await,
            SequenceElement::Callback(callback) => callback(),
        }
    }

    current_text
}

async fn perform_edits(
    displayed: &mut Signal<String>,
    edits: &[String],
    speed: u64,
    deletion_speed: u64,
    omit_deletion_animation: bool,
    current_text: &mut String,
) {
    let filtered_edits = if omit_deletion_animation {
        omit_deletion_edits(edits)
    } else {
        edits.to_vec()
    };

    for snippet in filtered_edits {
        let op = if snippet.is_empty() || current_text.chars().count() > snippet.chars().count() {
            Operation::Deletion
        } else {
            Operation::Writing
        };

        *current_text = snippet.clone();
        displayed.set(snippet);

        let base_delay = match op {
            Operation::Writing => speed,
            Operation::Deletion => deletion_speed,
        };
        wait(jittered_delay(base_delay)).await;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Writing,
    Deletion,
}

async fn wait(ms: u64) {
    TimeoutFuture::new(ms.min(u32::MAX as u64) as u32).await;
}

pub(crate) fn default_splitter() -> StringSplitter {
    Rc::new(|text: &str| text.chars().map(|c| c.to_string()).collect())
}

pub(crate) fn first_string(sequence: &[SequenceElement]) -> Option<String> {
    sequence.iter().find_map(|item| match item {
        SequenceElement::Text(text) => Some(text.clone()),
        _ => None,
    })
}

pub(crate) fn normalize_speed(speed: Speed) -> u64 {
    match speed {
        Speed::Preset(value) => value.abs_diff(100),
        Speed::KeyStrokeDelayInMs(value) => value,
    }
}

fn jittered_delay(base: u64) -> u64 {
    if base == 0 {
        return 0;
    }

    let jitter = fastrand::f64() - 0.5;
    ((base as f64) + (base as f64 * jitter)).max(0.0).round() as u64
}

pub(crate) fn final_class_name(cursor: bool, class: Option<&str>) -> String {
    match (cursor, class.filter(|value| !value.is_empty())) {
        (true, Some(class)) => format!("{CURSOR_CLASS} {class}"),
        (true, None) => CURSOR_CLASS.to_string(),
        (false, Some(class)) => class.to_string(),
        (false, None) => String::new(),
    }
}

fn edits_for_transition(start: &str, end: &str, splitter: &StringSplitter) -> Vec<String> {
    let overlap = get_overlap(start, end);
    deleter(start, splitter, overlap)
        .into_iter()
        .chain(writer(end, splitter, overlap))
        .collect()
}

fn writer(text: &str, splitter: &StringSplitter, mut start_index: usize) -> Vec<String> {
    let split_text = splitter(text);
    let end_index = split_text.len();
    let mut snippets = Vec::new();

    while start_index < end_index {
        start_index += 1;
        snippets.push(split_text[..start_index].join(""));
    }

    snippets
}

fn deleter(text: &str, splitter: &StringSplitter, start_index: usize) -> Vec<String> {
    let split_text = splitter(text);
    let mut end_index = split_text.len();
    let mut snippets = Vec::new();

    while end_index > start_index {
        end_index -= 1;
        snippets.push(split_text[..end_index].join(""));
    }

    snippets
}

fn get_overlap(start: &str, end: &str) -> usize {
    let mut index = 0;

    for (start_char, end_char) in start.chars().zip(end.chars()) {
        if start_char != end_char {
            return index;
        }
        index += 1;
    }

    index.min(start.chars().count()).min(end.chars().count())
}

fn omit_deletion_edits(edits: &[String]) -> Vec<String> {
    let mut slice_point = 0;

    for i in 1..edits.len() {
        let prev_len = edits[i - 1].chars().count();
        let curr_len = edits[i].chars().count();

        if curr_len > prev_len || edits[i].is_empty() {
            slice_point = i;
            break;
        }
    }

    edits[slice_point..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speed_presets_match_react_normalization() {
        assert_eq!(normalize_speed(Speed::Preset(40)), 60);
        assert_eq!(normalize_speed(Speed::Preset(99)), 1);
        assert_eq!(normalize_speed(Speed::Preset(1)), 99);
        assert_eq!(normalize_speed(Speed::KeyStrokeDelayInMs(125)), 125);
    }

    #[test]
    fn overlap_finds_common_prefix() {
        assert_eq!(
            get_overlap("We produce food for Mice", "We produce food for Hamsters"),
            20
        );
        assert_eq!(get_overlap("abc", "abcde"), 3);
        assert_eq!(get_overlap("abc", "xyz"), 0);
    }

    #[test]
    fn transition_deletes_to_overlap_then_writes() {
        let splitter = default_splitter();
        let edits = edits_for_transition("abc", "abd", &splitter);
        assert_eq!(edits, vec!["ab".to_string(), "abd".to_string()]);
    }

    #[test]
    fn omit_deletion_starts_at_deletion_end_state() {
        let edits = vec![
            "abc".to_string(),
            "ab".to_string(),
            "a".to_string(),
            "ax".to_string(),
        ];

        assert_eq!(omit_deletion_edits(&edits), vec!["ax".to_string()]);
    }

    #[test]
    fn class_name_matches_cursor_and_custom_class_settings() {
        assert_eq!(final_class_name(true, None), CURSOR_CLASS);
        assert_eq!(
            final_class_name(true, Some("hero")),
            format!("{CURSOR_CLASS} hero")
        );
        assert_eq!(final_class_name(false, Some("hero")), "hero");
        assert_eq!(final_class_name(false, None), "");
    }
}
