use crate::{Blog, Home, Photography, Projects}; //import Pages and components
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/projects")]
    Projects {},
    #[route("/photos")]
    Photography {},
    #[route("/blog")]
    Blog {},
}
