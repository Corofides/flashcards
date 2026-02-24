use yew::{Callback, Properties, component, html, Html};


#[derive(Properties, PartialEq)]
pub struct ActionButtonProperties {
    pub enabled: bool,
    pub aria_label: String,
    pub onclick: Callback<yew::MouseEvent>,
    pub icon: String,
}

#[component]
pub fn ActionButton(ActionButtonProperties { enabled, aria_label, onclick, icon }: &ActionButtonProperties) -> Html {
    let aria_label = aria_label.clone();

    html! {
        <button disabled={!enabled} class="nav-btn" aria-label={aria_label} onclick={onclick}>{ icon }</button>
    }
}
