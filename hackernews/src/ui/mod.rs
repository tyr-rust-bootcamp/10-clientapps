#![allow(non_snake_case)]

mod story_comment;
mod story_item;

use crate::api::get_top_stories;
use dioxus::prelude::*;
use story_item::StoryItem;

pub fn App() -> Element {
    rsx! {
      main { class: "flex w-full h-full shadow-lg rounded-3xl",
        section { class: "flex flex-col w-4/12 h-full pt-3 overflow-y-scroll bg-gray-50",
          Stories {}
        }
        section { class: "flex flex-col w-8/12 px-4 bg-white rounded-r-3xl",
          section {
            ul {}
          }
        }
      }
    }
}

#[component]
fn Stories() -> Element {
    let stories = use_resource(move || get_top_stories(20));
    match &*stories.read_unchecked() {
        Some(Ok(stories)) => rsx! {
          ul {
            for story in stories {
              StoryItem { story: story.clone() }
            }
          }
        },
        Some(Err(err)) => rsx! {
          div { class: "mt-6 text-red-500",
            p { "Failed to fetch stories" }
            p { "{err}" }
          }
        },
        None => rsx! {
          div { class: "mt-6",
            p { "Loading stories..." }
          }
        },
    }
}
