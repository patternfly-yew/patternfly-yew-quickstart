use crate::example::ExamplePage;

use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
            <ExamplePage title="Patternfly Yew Quickstart">
                {"Pick an example on the left"}
            </ExamplePage>
        </>
    }
}
