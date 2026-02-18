use crate::example::ExamplePage;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(Counter)]
pub fn counter() -> Html {
    let counter = use_state_eq(|| 0);

    let onclick = use_callback(counter.clone(), |_, counter| counter.set(**counter + 1));

    html! (
        <ExamplePage title="Counting clicks">
            <Gallery gutter=true>
                <Card>
                    <CardTitle>{ "Clicks" }</CardTitle>
                    <CardBody>
                        <p>{ *counter }</p>
                        <Button
                            label="Add One"
                            align={Align::Start}
                            icon={Icon::PlusCircle}
                            variant={ButtonVariant::Link}
                            {onclick}
                        />
                    </CardBody>
                </Card>
            </Gallery>
        </ExamplePage>
    )
}
