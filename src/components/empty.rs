use crate::{example, example::Example};

use patternfly_yew::*;
use yew::prelude::*;

pub struct EmptyStateExample {
    link: ComponentLink<Self>,
}

impl Component for EmptyStateExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let example1 = example! {"Empty state" =>
            <EmptyState
                title="Empty state"
                icon=Icon::Cubes
                primary=Action::new("Push me", self.link.callback(|_|{}))
                secondaries=vec![
                    Action::new("Try me", self.link.callback(|_|{})),
                    Action::new("Me too", self.link.callback(|_|{})),
                    Action::new("Here, here", self.link.callback(|_|{})),
                ]
                >
                {"This section should explain why the state is empty, and what you can do next."}
            </EmptyState>
        };

        let example2 = example! {"Empty state (XLarge)" =>
            <EmptyState
                title="Empty state"
                icon=Icon::Cubes
                size=Size::XLarge
                primary=Action::new("Push me", self.link.callback(|_|{}))
                secondaries=vec![
                    Action::new("Try me", self.link.callback(|_|{})),
                    Action::new("Me too", self.link.callback(|_|{})),
                    Action::new("Here, here", self.link.callback(|_|{})),
                ]
                >
                {"This section should explain why the state is empty, and what you can do next."}
            </EmptyState>
        };

        html! {
            <>
                <Example title="Empty state">
                    {example1}
                    {example2}
                </Example>
            </>
        }
    }
}
