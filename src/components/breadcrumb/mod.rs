use crate::{example::ExamplePage, example2};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(BreadcrumbExample)]
pub fn breadcrumb() -> Html {
    let example1 = example2! ("Breadcrumb" => "breadcrumb.1.example");
    let example2 = example2! ("Breadcrumb (clickable root)" => "breadcrumb.2.example");

    html! {
        <>
            <ExamplePage title="Breadcrumb">
                {example1}
                {example2}
            </ExamplePage>
        </>
    }
}
