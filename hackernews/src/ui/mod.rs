#![allow(non_snake_case)]

mod comments;
mod stories;

use crate::StoryData;
use comments::Comments;
use dioxus::prelude::*;
use stories::Stories;

#[derive(Debug, Clone)]
pub enum CommentsState {
    Unset,
    Loading,
    Loaded(StoryData),
}

pub fn App() -> Element {
    use_context_provider(|| Signal::new(CommentsState::Unset));
    rsx! {
      main { class: "flex w-full h-full shadow-lg rounded-3xl",
        section { class: "flex flex-col w-4/12 h-full pt-3 overflow-y-scroll bg-gray-50",
          Stories {}
        }
        section { class: "flex flex-col w-8/12 px-4 bg-white rounded-r-3xl", Comments {} }
      }
    }
}
