#![allow(non_snake_case)]

use crate::StoryItem;
use dioxus::prelude::*;

#[component]
pub fn StoryItem(story: StoryItem) -> Element {
    rsx! {
      li { class: "px-3 py-5 transition border-b hover:bg-indigo-100",
        a { href: "#", class: "flex items-center justify-between",
          h3 { class: "text-lg font-semibold", "{story.title}" }
          p { class: "text-gray-400 text-md" }
        }
        div { class: "italic text-gray-400 text-md",
          span { "{story.score} points by {story.by} {story.time} | " }
          a { href: "#", "{story.kids.len()} comments" }
        }
      }
    }
}
