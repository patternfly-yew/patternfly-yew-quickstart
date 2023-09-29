use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

pub struct EmptyStateExample {}

impl Component for EmptyStateExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let example_a = example! ("Simple Empty State" => "simple_empty_state.1.example");
        let example_b = example! ("Simple Empty state (XLarge)" => "simple_empty_state.2.example");

        let example1 = example! ("Basic" => "empty_state.1.example");
        let example2 = example! ("Extra small" => "empty_state.2.example");
        let example3 = example! ("Small" => "empty_state.3.example");
        let example4 = example! ("Large" => "empty_state.4.example");
        let example5 = example! ("Extra large" => "empty_state.5.example");
        let example6 = example! ("Spinner" => "empty_state.6.example");
        let example7 = example! ("No match found" => "empty_state.7.example");
        let example8 = example! ("Custom icon color" => "empty_state.8.example");

        html! {
            <>
                <ExamplePage title="Empty state">
                    {example_a}
                    {example_b}

                    {example1}
                    {example2}
                    {example3}
                    {example4}
                    {example5}
                    {example6}
                    {example7}
                    {example8}
                </ExamplePage>
            </>
        }
    }
}
