use crate::example::{ExamplePage, NavigationDemo, NavigationExample};
use yew::prelude::*;

#[function_component(BackToTopExample)]
pub fn back_to_top_example() -> Html {
    html! {
        <ExamplePage title="Back to top">
            <p>{"Scroll inside each preview to test its button independently."}</p>
            <NavigationExample demo={NavigationDemo::BackToTop} />
            <NavigationExample demo={NavigationDemo::BackToTopAlwaysVisible} />
            <NavigationExample demo={NavigationDemo::BackToTopCustomTitle} />
        </ExamplePage>
    }
}
