use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(BrandExample)]
pub fn brand() -> Html {
    let example1 = example! ("Basic" => "brand.1.example");

    html! {
        <>
            <ExamplePage title="Brand">{ example1 }</ExamplePage>
        </>
    }
}
