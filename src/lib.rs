#![recursion_limit = "256"]

mod index;
mod counter;

use counter::*;
use index::*;

use patternfly_yew::*;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

struct Model {
    link: ComponentLink<Self>,
}

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/counter"]
    Counter,
    #[to = "/"]
    Index,
}

impl Component for Model {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
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
                    <NavList>
                        <NavRouterItem<AppRoute> to=AppRoute::Index>{"Index"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Counter>{"Counter"}</NavRouterItem<AppRoute>>
                    </NavList>
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
                        }
                    })
                />
            </Page>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
