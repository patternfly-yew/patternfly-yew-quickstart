use crate::{example2, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(SpinnerExample)]
pub fn spinner() -> Html {
        let example1 = example2! ("Basic" => "spinner.1.example");
        let example2 = example2! ("Size variations" => "spinner.2.example");
        let example3 = example2! ("Custom size" => "spinner.3.example");

        html! { 
            <>
            <ExamplePage title="Spinners">
                {example1}
                {example2}
                {example3}
            </ExamplePage>
            </>
        }
}
