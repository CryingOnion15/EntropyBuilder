use dioxus::prelude::*;

pub fn Loader() -> Element {
    rsx! {
        div { style: "width: 100%; height: 100%; background-color: pink;", "LOADING!!!!" }
    }
}
