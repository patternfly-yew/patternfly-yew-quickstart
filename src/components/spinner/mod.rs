use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(SpinnerExample)]
pub fn spinner() -> Html {
    let example1 = example! ("Basic" => "spinner.1.example");
    let example2 = example! ("Size variations" => "spinner.2.example");
    let example3 = example! ("Custom size" => "spinner.3.example");

    html! {
        <>
            <ExamplePage title="Spinners">{ example1 }{ example2 }{ example3 }</ExamplePage>
        </>
    }
}
