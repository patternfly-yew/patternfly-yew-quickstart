use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(BreadcrumbExample)]
pub fn breadcrumb() -> Html {
    let example1 = example! ("Breadcrumb" => "breadcrumb.1.example");
    let example2 = example! ("Breadcrumb (clickable root)" => "breadcrumb.2.example");

    html! {
        <>
            <ExamplePage title="Breadcrumb">{ example1 }{ example2 }</ExamplePage>
        </>
    }
}
