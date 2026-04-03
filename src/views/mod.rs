mod layout;
use layout::LayoutView;

mod sidebar;
use sidebar::SidebarView;

mod chat;
use chat::ChatView;

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
	#[layout(LayoutView)]
	#[route("/")]
	ChatView {},
}