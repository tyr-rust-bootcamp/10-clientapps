#![allow(non_snake_case)]

use crate::StoryItem;
use dioxus::prelude::*;

#[component]
pub fn StoryComment(data: StoryItem) -> Element {
    rsx! {
      li {
        article { class: "mt-8 leading-7 tracking-wider text-gray-500",
          p { "Hi Akhil," }
          p {
            "Design and develop enterprise-facing UI and consumer-facing UI as well as\n      REST API\n      backends.Work with\n      Product Managers and User Experience designers to create an appealing user experience for desktop web and\n      mobile web."
          }
          footer { class: "mt-12",
            p { "Thanks & Regards," }
            p { "Alexandar" }
          }
        }
      }
    }
}
