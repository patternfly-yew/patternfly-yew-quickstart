use crate::{example, example::ExamplePage, layouts::LayoutItem};
use patternfly_yew::prelude::*;
use yew::prelude::*;

pub struct GridExample {}

impl Component for GridExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let example1 = example!("Grid" => "grid.1.example");
        let example2 = example!("Grid (gutter)" => "grid.2.example");
        html! {
            <>
                <ExamplePage title="Grid Layout">{ example1 }{ example2 }</ExamplePage>
            </>
        }
    }
}
