use dioxus::prelude::*;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            h1 { "Blog Post #{id}" }
            p { "This is a blog post with ID {id}" }
        }
    }
}
