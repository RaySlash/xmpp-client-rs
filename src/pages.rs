use crate::routes::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn Blog(cx: Scope) -> Element {
    render! {
        Navbar {},
        "My Blog posts"
    }
}

#[component]
pub fn Home(cx: Scope) -> Element {
    render! {
        Navbar {},
        div {
            class: "main-title",
            "Hi, I am Steve Mathew Joy"
        }
    }
}

#[component]
pub fn Projects(cx: Scope) -> Element {
    render! {
        Navbar {},
        div {

        }
    }
}

#[component]
pub fn Photography(cx: Scope) -> Element {
    render! {
        Navbar {},
        div {
        }
    }
}

// Sub-Components inside pages
#[component]
pub fn Navbar(cx: Scope) -> Element {
    render! {
        div {
            Link {
                to: Route::Home {},
                "Home"
            },
            Link {
                to: Route::Projects{},
                "Projects"
            },
            Link {
                to: Route::Photography {},
                "Photography"
            },
            Link {
                to: Route::Blog {},
                "Blog"
            },

        }
    }
}
