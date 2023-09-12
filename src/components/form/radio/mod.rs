use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(FormRadioExample)]
pub fn view() -> Html {
    let example1 = example! {"Basic Radio" => "form.1.example" };
    let example2 = example! {"Reversed" => "form.2.example" };
    let example3 = example! {"Standalone" => "form.3.example" };
    let example4 = example! {"Description" => "form.4.example" };
    let example5 = example! {"Body" => "form.5.example" };
    let example6 = example! {"Description & body" => "form.6.example" };

    html! (
        <ExamplePage title="Radio">
            { example1 }
            { example2 }
            { example3 }
            { example4 }
            { example5 }
            { example6 }
        </ExamplePage>
    )
}
