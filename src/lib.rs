#![recursion_limit = "512"]

mod components;
mod counter;
mod example;
mod index;

use counter::*;
use index::*;

use patternfly_yew::*;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

struct Model {}

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/components/badge"]
    Badge,
    #[to = "/components/clipboard"]
    Clipboard,
    #[to = "/components/form"]
    Form,
    #[to = "/components/table"]
    Table,
    #[to = "/components/tooltip"]
    Tooltip,
    #[to = "/counter"]
    Counter,
    #[to = "/"]
    Index,
}

impl Component for Model {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let sidebar = html_nested! {
            <PageSidebar>
                <Nav>
                    <NavGroup title="Basics">
                        <NavRouterItem<AppRoute> to=AppRoute::Index>{"Index"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Counter>{"Counter"}</NavRouterItem<AppRoute>>
                    </NavGroup>
                    <NavGroup title="Components">
                        <NavRouterItem<AppRoute> to=AppRoute::Badge>{"Badge"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Clipboard>{"Clipboard"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Form>{"Form"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Table>{"Table"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Tooltip>{"Tooltip"}</NavRouterItem<AppRoute>>
                    </NavGroup>
                </Nav>
            </PageSidebar>
        };
        let _header_tools = html! { {"Foo"} };

        html! {
            <Page
                logo={html_nested!{
                    <Logo src="https://www.patternfly.org/assets/images/PF-Masthead-Logo.svg" alt="Patternfly Logo" />
                }}
                sidebar=sidebar
                >
                <Router<AppRoute, ()>
                    redirect = Router::redirect(|_|AppRoute::Index)
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Counter => html!{<Counter/>},
                            AppRoute::Index => html!{<Index/>},
                            AppRoute::Badge => html!{<components::BadgeExample/>},
                            AppRoute::Clipboard => html!{<components::ClipboardExample/>},
                            AppRoute::Form => html!{<components::FormExample/>},
                            AppRoute::Table => html!{<components::TableExample/>},
                            AppRoute::Tooltip => html!{<components::TooltipExample/>},
                        }
                    })
                />
            </Page>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(Default::default());
    App::<Model>::new().mount_to_body();
}
