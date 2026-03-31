pub mod sidebar;
pub use sidebar::SidebarView;

pub mod home;
pub use home::HomeView;

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
	#[layout(SidebarView)]
	#[route("/")]
	HomeView {},
}