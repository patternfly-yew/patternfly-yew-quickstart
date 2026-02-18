use crate::{example, example::ExamplePage};

use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(DividerExample)]
pub fn divider() -> Html {
    let example1 = example! ("Divider (default)" => "divider.1.example");
    let example2 = example! ("Divider (with type)" => "divider.2.example");
    let example3 = example! ("Divider (vertical)" => "divider.3.example");
    let example4 = example! ("Divider (with inset)" => "divider.4.example");
    let example5 = example! ("Divider (vertical with inset)" => "divider.5.example");
    // let example5 = example! ("Chip Groups" => "chip.5.example");

    html! {
        <>
            <ExamplePage title="Divider">
                { example1 }
                { example2 }
                { example3 }
                { example4 }
                { example5 }
                // Fixme: The below examples see issue: #57
                // Using a list component
                // Using a flex component
                // With Flex and Responsive
                // <Flex>
                //     <FlexItem>{"first item"}</FlexItem>
                //     <Divider orientation={DividerOrientation::Vertical.lg()}/>
                //     <FlexItem>{"second item"}</FlexItem>
                // </Flex>
            </ExamplePage>
        </>
    }
}
