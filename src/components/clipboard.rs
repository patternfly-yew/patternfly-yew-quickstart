use crate::{example::Example, example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

pub struct ClipboardExample {}

impl Component for ClipboardExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let example1 = example2! ("Clipboard" => "clipboard.1.example");

        html! {
            <>
                <ExamplePage title="Clipboard">
                    {example1}
                </ExamplePage>
            </>
        }
    }
}
