use crate::example::ExamplePage;

use yew::prelude::*;

pub struct Index {}

impl Component for Index {
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
        html! {
            <>
                <ExamplePage title="Patternfly Yew Quickstart">
                    {"Pick an example on the left"}
                </ExamplePage>
            </>
        }
    }
}
