use crate::{example::Example, example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

pub struct EmptyStateExample {
    link: ComponentLink<Self>,
}

impl Component for EmptyStateExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let example1 = example2! ("Empty state" => "empty.1.example");
        let example2 = example2! ("Empty state (XLarge)" => "empty.2.example");

        html! {
            <>
                <ExamplePage title="Empty state">
                    {example1}
                    {example2}
                </ExamplePage>
            </>
        }
    }
}
