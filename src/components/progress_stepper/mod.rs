use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(ProgressStepperExample)]
pub fn progress() -> Html {
    let example_1 = example! ("Basic" => "progress_stepper.1.example");
    let example_2 = example! ("Basic with descriptions" => "progress_stepper.2.example");
    let example_3 = example! ("Vertical, horizontal responsive" => "progress_stepper.3.example");
    let example_4 = example! ("Center aligned with descriptions" => "progress_stepper.4.example");
    let example_5 = example! ("Center aligned, vertical" => "progress_stepper.5.example");
    let example_6 = example! ("Vertical with descriptions" => "progress_stepper.6.example");
    let example_7 = example! ("Compact" => "progress_stepper.7.example");
    let example_8 = example! ("Compact Vertical" => "progress_stepper.8.example");
    let example_9 = example! ("Compact, vertical responsive" => "progress_stepper.9.example");
    let example_10 = example! ("Compact, vertical, centered" => "progress_stepper.10.example");
    let example_11 = example! ("Compact, centered" => "progress_stepper.11.example");
    let example_12 = example! ("Basic with an issue" => "progress_stepper.12.example");
    let example_13 = example! ("Basic with a failure" => "progress_stepper.13.example");
    let example_14 = example! ("With help text" => "progress_stepper.14.example");

    html! {
        <ExamplePage title="Progress Stepper">
            { example_1 }
            { example_2 }
            { example_3 }
            { example_4 }
            { example_5 }
            { example_6 }
            { example_7 }
            { example_8 }
            { example_9 }
            { example_10 }
            { example_11 }
            { example_12 }
            { example_13 }
            { example_14 }
        </ExamplePage>
    }
}
