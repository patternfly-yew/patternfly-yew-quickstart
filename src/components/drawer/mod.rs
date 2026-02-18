use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(DrawerExample)]
pub fn drawer() -> Html {
    let example1 = example! ("Drawer" => "drawer.1.example");
    let example2 = example! ("Drawer (Left)" => "drawer.2.example");
    let example3 = example! ("Drawer (Bottom)" => "drawer.3.example");
    let example4 = example! ("Drawer (Additional Section)" => "drawer.4.example");

    html! {
        <>
            <ExamplePage title="Drawer">
                { example1 }
                { example2 }
                { example3 }
                { example4 }
            </ExamplePage>
        </>
    }
}
