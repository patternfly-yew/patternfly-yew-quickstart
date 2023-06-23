use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(ContextSelectorExample)]
pub fn example() -> Html {
    let examples: Vec<Html> = vec![
        example! ("Context Selector" => "context_selector.1.example"),
        example! ("Context Selector (form)" => "context_selector.2.example"),
    ];

    html! {
        <>
            <ExamplePage title="Context Selector">
                <Alert inline=true title="AppLauncher Deprecated" r#type={AlertType::Warning}>
                    {"The ContextSelector component has been deprecated by PatternFly."}
                    <br />
                    {"See "}
                    <a href="https://pf5.patternfly.org/components/menus/context-selector">
                        {"ContextSelector component page"}
                    </a>
                    {" in the PatternFly documents for more information."}
                </Alert>
                { for examples }
            </ExamplePage>
        </>
    }
}
