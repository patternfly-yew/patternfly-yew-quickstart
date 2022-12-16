use crate::{example::ExamplePage, example2};
use patternfly_yew::*;
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

        let examples: Vec<Html> = vec![example2! ("Tabs" => "tabs.1.example")];

        html! {
            <>
                <Scope<crate::app::Component, TabRoutes> mapper={crate::app::Component::mapper_tabs}>
                    <ExamplePage title="Tabs">
                        <PageSection variant={PageSectionVariant::Light} limit_width=true>
                            { for examples }
                        </PageSection>
                    </ExamplePage>
                </Scope<crate::app::Component, TabRoutes>>
            </>
        }
    }
}
