use crate::example::{ExamplePage, NavigationDemo, NavigationExample};
use yew::prelude::*;

#[function_component(SkipToContentExample)]
pub fn skip_to_content_example() -> Html {
    html! {
        <ExamplePage title="Skip to content">
            <p>{"Tab into a preview: its first focusable element is the skip link. Press Enter to bypass its navigation and focus the content."}</p>
            <NavigationExample demo={NavigationDemo::SkipToContent} />
            <NavigationExample demo={NavigationDemo::SkipToContentCustomText} />
        </ExamplePage>
    }
}
