use crate::{
    example::{Example, ExamplePage},
    example2,
    layouts::LayoutItem,
};
use patternfly_yew::*;
use yew::prelude::*;

pub struct StackExample {}

impl Component for StackExample {
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
        let example1 = example2!("Stack" => "stack.1.example");
        let example2 = example2!("Stack (gutter)" => "stack.2.example");
        html! {
            <>
                <ExamplePage title="Stack Layout">
                    { example1 }
                    { example2 }
                </ExamplePage>
            </>
        }
    }
}
