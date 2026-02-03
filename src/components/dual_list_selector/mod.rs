use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::html::IntoPropValue;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
struct Fruit {
    name: String,
    id: usize,
}

impl Fruit {
    fn new(name: &str, id: usize) -> Self {
        Self {
            name: name.to_string(),
            id,
        }
    }
}

impl IntoPropValue<Html> for Fruit {
    fn into_prop_value(self) -> Html {
        html! { {self.name.clone() }}
    }
}

impl IntoPropValue<Html> for &Fruit {
    fn into_prop_value(self) -> Html {
        html! { {self.name.clone() }}
    }
}

#[component(DualListSelectorExample)]
pub fn view() -> Html {
    let example1 = example! {"Basic" => "dual_list_selector.1.example"};
    let example2 = example! {"Advanced" => "dual_list_selector.2.example"};
    let example3 = example! {"Disabled" => "dual_list_selector.3.example"};
    html! (
        <ExamplePage title="DualListSelector">
            {example1}
            {example2}
            {example3}
        </ExamplePage>
    )
}
