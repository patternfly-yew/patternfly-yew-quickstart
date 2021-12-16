use crate::{example::ExamplePage, example2};
use patternfly_yew::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Switch, Debug, Clone, Copy, Eq, PartialEq)]
pub enum TabRoutes {
    #[to = "/foo"]
    Foo,
    #[to = "/bar"]
    Bar,
    #[to = "/baz"]
    Baz,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub current: TabRoutes,
}

pub struct TabsExample {}

type MyTabsRouter = TabsRouter<crate::app::AppRoute, TabRoutes>;

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
                <ExamplePage title="Tabs">
                    <PageSection variant={PageSectionVariant::Light} limit_width=true>
                        { for examples }
                    </PageSection>
                </ExamplePage>
            </>
        }
    }
}
