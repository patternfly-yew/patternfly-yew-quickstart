use crate::{example, example::Example};

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
        let example1 = example! {"Clipboard" =>
            <Clipboard value="Foo bar"/>
        };

        html! {
            <>
                <Example title="Clipboard">
                    {example1}
                </Example>
            </>
        }
    }
}
