use crate::routes::Route;
use chrono::prelude::*;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fi_icons, Icon};
use dioxus_router::prelude::*;
use emojis::Emoji;

#[component]
pub fn Blog(cx: Scope) -> Element {
    render! {
        Navbar {}
        "My Blog posts"
    }
}

#[component]
pub fn Home(cx: Scope) -> Element {
    let wave_emoji: &Emoji = emojis::get_by_shortcode("wave").expect("Failed to load emoji");

    render! {
        Navbar {}
        div {
            class: "intro",
            div {
                class: "name",
                p { "{wave_emoji} Hi, I am" }
                h1 { "Steve Mathew Joy" }
            }
            div {
                class: "tagline",
                p { "Tell me more about computing and cars!" }
                p { "Join the ٩( ╹ o ╹)۶" }
            }
        }
        Footer {}
    }
}

#[component]
pub fn Projects(cx: Scope) -> Element {
    render! {
        Navbar {}
        div {
        }
    }
}

#[component]
pub fn Photography(cx: Scope) -> Element {
    render! {
        Navbar {}
        div {
        }
    }
}

// Sub-Components inside pages
#[component]
pub fn Navbar(cx: Scope) -> Element {
    render! {
        header {
            class: "navbar",
            div {
                class: "nav-routes",
                Link { to: Route::Home {}, "Home" }
                Link { to: Route::Projects {}, "Projects" }
                Link { to: Route::Photography {}, "Photography" }
                Link { to: Route::Blog {}, "Blog" }
            }
            div {
                class: "nav-outbound",
                Link {
                    to: "https://github.com/rayslash",
                    Icon {
                        width: 25,
                        height: 25,
                        icon: fi_icons::FiGithub
                    }
                }
            }
        }
    }
}

pub fn Footer(cx: Scope) -> Element {
    let year = Utc::now().year();
    render! {
        footer {
            div {
                class: "credits",
                "Made with love @ {year}"
            }
        }
    }
}
