use dioxus::prelude::*;

pub fn ProjectExplorer() -> Element {
    rsx! {
        div { style: "width: 100%; height: 100%, background-color: green", "Project View" }
    }
}
