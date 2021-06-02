use crate::{example::Example, example::ExamplePage, example2};
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

pub struct TabsExample {
    props: Props,
}

type MyTabsRouter = TabsRouter<crate::AppRoute, TabRoutes>;

impl Component for TabsExample {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        log::info!("Route: {:?}", self.props.current);

        let examples: Vec<Html> = vec![example2! ("Tabs" => "tabs.1.example")];

        html! {
            <>
                <ExamplePage title="Tabs">
                    <PageSection variant=PageSectionVariant::Light limit_width=true>
                        { for examples }
                    </PageSection>
                </ExamplePage>
            </>
        }
    }
}
