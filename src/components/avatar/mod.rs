use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(AvatarExample)]
pub fn avatar() -> Html {
    let example1 = example! ("Basic" => "avatar.1.example");
    let example2 = example! ("Size Variations" => "avatar.2.example");
    let example3 = example! ("Border Styles" => "avatar.3.example");

    html! {
        <>
            <ExamplePage title="Avatar">{ example1 }{ example2 }{ example3 }</ExamplePage>
        </>
    }
}
