use crate::example::ExamplePage;

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(Counter)]
pub fn counter() -> Html {
    let counter = use_state_eq(|| 0);

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    let title = html! {<>
        {"Clicks"}
    </>};

    html! (
        <ExamplePage title="Counting clicks">
            <Gallery gutter=true>
                <Card
                    selection={CardSelection::Selectable {selected: true}}
                    title={title}
                    >
                    <p>{ *counter }</p>
                </Card>
            </Gallery>
            <Form>
                <Button label="Add One" align={Align::Start} icon={Icon::PlusCircle} variant={ButtonVariant::Link} {onclick}/>
            </Form>
        </ExamplePage>
    )
}
