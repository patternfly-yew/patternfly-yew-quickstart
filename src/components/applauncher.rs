use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(AppLauncherExample)]
pub fn app_launcher_example() -> Html {
    let example1 = example2! ("AppLauncher" => "applauncher.1.example");

    html! {
        <>
            <ExamplePage title="AppLauncher">
                {example1}
            </ExamplePage>
        </>
    }
}
