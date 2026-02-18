use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::prelude::{Switch as RouterSwitch, *};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Target)]
pub enum TabRoutes {
    Foo,
    Bar,
    Baz,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub current: TabRoutes,
}

pub struct TabsExample {}

impl Component for TabsExample {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::info!("Route: {:?}", ctx.props().current);

        let example1 = example! ("Tabs (Router)" => "tabs.1.example");
        let example2 = example! ("Tabs (Simple)" => "tabs.2.example");
        let example3 = example! ("Tabs (Simple, detached, boxed)" => "tabs.3.example");

        html! {
            <Scope<crate::app::Component, TabRoutes> mapper={crate::app::Component::mapper_tabs}>
                <ExamplePage title="Tabs">{ example1 }{ example2 }{ example3 }</ExamplePage>
            </Scope<crate::app::Component, TabRoutes>>
        }
    }
}
