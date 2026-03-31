mod layout;
use layout::LayoutView;

mod sidebar;
use sidebar::SidebarView;

mod home;
use home::HomeView;

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
	#[layout(LayoutView)]
	#[route("/")]
	HomeView {},
}