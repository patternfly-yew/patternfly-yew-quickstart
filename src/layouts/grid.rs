use crate::{example::ExamplePage, example2, layouts::LayoutItem};
use patternfly_yew::*;
use yew::prelude::*;

pub struct GridExample {}

impl Component for GridExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let example1 = example2!("Grid" => "grid.1.example");
        let example2 = example2!("Grid (gutter)" => "grid.2.example");
        html! {
            <>
                <ExamplePage title="Grid Layout">
                    { example1 }
                    { example2 }
                </ExamplePage>
            </>
        }
    }
}
