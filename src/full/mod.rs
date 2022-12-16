mod login_page;

pub use self::login_page::*;

use crate::example::*;
use patternfly_yew::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub url: String,
}

#[function_component(FullPageExample)]
pub fn full_page_example(props: &Props) -> Html {
    let url = props.url.clone();

    let onclick = move |_| {
        gloo_utils::window()
            .open_with_url_and_target(&url, "patternfly-yew-example")
            .ok();
    };

    html! {
        <>
            <ExamplePage title="Login Page">
                <Button variant={Variant::Primary} icon={Icon::ExternalLinkAlt} align={Align::Start} label="Open" {onclick} />
            </ExamplePage>
        </>
    }
}
