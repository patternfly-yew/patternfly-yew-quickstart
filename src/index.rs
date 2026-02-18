use crate::example::ExamplePage;

use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
    let subtitle = Html::from_html_unchecked(
        r#"<div>
<p>
This project acts both as a showcase for <strong>PatternFly Yew</strong>, as well as a quick-start project template.
</p>
</div>"#
            .into(),
    );
    html! {
        <>
            <ExamplePage title="Patternfly Yew Quickstart" {subtitle}>
                { "Pick an example on the left to learn more." }
            </ExamplePage>
        </>
    }
}
