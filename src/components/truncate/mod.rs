use crate::example;
use crate::example::ExamplePage;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(TruncateExample)]
pub fn truncate_example() -> Html {
    let example1 = example! ("Default" => "truncate.1.example");
    let example2 = example! ("Start" => "truncate.2.example");
    let example3 = example! ("Middle" => "truncate.3.example");
    let example4 = example! ("Middle (explicit)" => "truncate.4.example");

    #[function_component(Resize)]
    fn resize(props: &ChildrenProperties) -> Html {
        html!(
            <div
                style="
                    resize: horizontal;
                    border: var(--pf-v6-global--BorderWidth--sm) solid var(--pf-v6-global--BorderColor--100);
                    padding: var(--pf-v6-global--spacer--md);
                    overflow: auto;
                    width: 350px;
                    max-width: 100%
                "
            >
                { props.children.clone() }
            </div>
        )
    }

    html! (
        <>
            <ExamplePage title="Truncate">
                { example1 }
                { example2 }
                { example3 }
                { example4 }
            </ExamplePage>
        </>
    )
}
