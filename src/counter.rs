use crate::example::ExamplePage;

use patternfly_yew::*;
use yew::prelude::*;

pub struct Counter {
    link: ComponentLink<Self>,
    value: i64,
}

#[derive(Clone, Debug)]
pub enum Msg {
    AddOne,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        log::info!("Clicks: {}", self.value);
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <ExamplePage title="Counting clicks">
                    <Gallery gutter=true>
                        <Card
                            selectable=true
                            selected=true
                            title={html_nested!{<>
                                {"Clicks"}
                            </>}}
                            >

                            <p>{ self.value }</p>

                        </Card>
                    </Gallery>
                    <Form>
                        <Button label="Add One" align=Align::Start icon=Icon::PlusCircleIcon variant=Variant::Link onclick=self.link.callback(|_| Msg::AddOne)/>
                    </Form>
                </ExamplePage>
            </>
        }
    }
}
