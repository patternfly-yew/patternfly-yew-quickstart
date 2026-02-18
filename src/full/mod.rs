mod login_page;

pub use self::login_page::*;

use crate::example::*;
use patternfly_yew::prelude::*;
use url::Url;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub url: String,
}

fn build_url(url: &str) -> Option<String> {
    let base = gloo_utils::document()
        .base_uri()
        .ok()
        .flatten()
        .and_then(|url| Url::parse(&url).ok())?;

    base.join(url).ok().map(|url| url.to_string())
}

#[function_component(FullPageExample)]
pub fn full_page_example(props: &Props) -> Html {
    let url = props.url.clone();
    let onclick = move |_| {
        let url = build_url(&url).unwrap_or_else(|| url.clone());
        let _ = gloo_utils::window().open_with_url_and_target(&url, "patternfly-yew-example");
    };

    html! {
        <>
            <ExamplePage title="Login Page">
                <Button
                    variant={ButtonVariant::Primary}
                    icon={Icon::ExternalLinkAlt}
                    align={Align::Start}
                    label="Open"
                    {onclick}
                />
            </ExamplePage>
        </>
    }
}

#[cfg(test)]
mod test {
    use url::Url;

    #[test]
    fn join_base() {
        let base =
            Url::parse("https://patternfly-yew.github.io/patternfly-yew-quickstart/").unwrap();
        let url = base.join("full/login").unwrap();
        assert_eq!(
            url.as_str(),
            "https://patternfly-yew.github.io/patternfly-yew-quickstart/full/login"
        );
    }
}
