use crate::example::{ExamplePage, NavigationDemo, NavigationExample};
use yew::prelude::*;

#[function_component(JumpLinksExample)]
pub fn jump_links_example() -> Html {
    html! {
        <ExamplePage title="Jump links">
            <p>{"Select a link or scroll inside a preview. The highlighted link follows the visible section."}</p>
            <NavigationExample demo={NavigationDemo::JumpLinks} />
            <NavigationExample demo={NavigationDemo::JumpLinksWithLabel} />
            <NavigationExample demo={NavigationDemo::JumpLinksHorizontal} />
            <NavigationExample demo={NavigationDemo::JumpLinksActive} />
            <NavigationExample demo={NavigationDemo::JumpLinksNested} />
        </ExamplePage>
    }
}
