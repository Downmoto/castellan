use dioxus::prelude::*;

#[component]
pub fn SidebarView() -> Element {
    let links = [
        ("Sessions", asset!("/assets/icons/tabler--folder-open.svg")),
        ("Sessions2", asset!("/assets/icons/tabler--folder-open.svg")),
        ("Sessions3", asset!("/assets/icons/tabler--folder-open.svg")),
    ];

    rsx! {
        div { class: "flex flex-col justify-center gap-2",
            for link in links {
                {
                    let link_name = link.0;
                    let link_icon = link.1;
                    rsx! {
                        img { src: link_icon, alt: "icon" }
                        div { "{link_name}" }
                    }
                }
            }
        }
    }
}
