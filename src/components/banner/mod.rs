use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(BannerExample)]
pub fn banner_example() -> Html {
    let example1 = example!("Basic" => "banner.1.example");
    let example2 = example!("Colors" => "banner.2.example");
    let example3 = example!("Status" => "banner.3.example");
    let example4 = example!("Pill" => "banner.4.example");
    let example5 = example!("Sticky" => "banner.5.example");
    html! {
        <ExamplePage title="Banner">
            {example1}
            {example2}
            {example3}
            {example4}
            {example5}
        </ExamplePage>
    }
}
