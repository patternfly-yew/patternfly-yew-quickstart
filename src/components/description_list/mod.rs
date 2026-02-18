use crate::{example, example::ExamplePage};

use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(DescriptionListExample)]
pub fn description_list() -> Html {
    let example1 = example! ("DescriptionList (default)" => "description_list.1.example");
    let example2 = example! ("DescriptionList (compact)" => "description_list.2.example");
    let example3 = example! ("DescriptionList (horizontal)" => "description_list.3.example");
    let example4 = example! ("DescriptionList (fluid)" => "description_list.4.example");

    html! {
        <>
            <ExamplePage title="DescriptionList">
                { example1 }
                { example2 }
                { example3 }
                { example4 }
            </ExamplePage>
        </>
    }
}
