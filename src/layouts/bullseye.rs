use crate::{example, example::Example};

use patternfly_yew::*;
use yew::prelude::*;

pub struct BullseyeExample {}

impl Component for BullseyeExample {
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
        let example1 = example! {"Bullseye" =>
            <Bullseye>
                <div>{"Item #1"}</div>
            </Bullseye>
        };

        html! {
            <>
                <Example title="Bullseye Layout">
                    {example1}
                </Example>
            </>
        }
    }
}
