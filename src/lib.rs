#![recursion_limit = "1024"]

mod components;
mod counter;
mod example;
mod index;
mod layouts;

use counter::*;
use index::*;

use patternfly_yew::*;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

struct Model {}

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum Component {
    #[to = "/badge"]
    Badge,
    #[to = "/clipboard"]
    Clipboard,
    #[to = "/empty"]
    EmptyState,
    #[to = "/form"]
    Form,
    #[to = "/label"]
    Label,
    #[to = "/popover"]
    Popover,
    #[to = "/table"]
    Table,
    #[to = "/tooltip"]
    Tooltip,
}

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum Layout {
    #[to = "/bullseye"]
    Bullseye,
    #[to = "/flex"]
    Flex,
    #[to = "/gallery"]
    Gallery,
}

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/components/{*:rest}"]
    Component(Component),
    #[to = "/layout/{*:rest}"]
    Layout(Layout),
    #[to = "/counter"]
    Counter,
    #[to = "/"]
    Index,
}

impl yew::Component for Model {
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
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Badge)>{"Badge"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Clipboard)>{"Clipboard"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::EmptyState)>{"Empty state"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Form)>{"Form"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Label)>{"Label"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Popover)>{"Popover"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Table)>{"Table"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Tooltip)>{"Tooltip"}</NavRouterItem<AppRoute>>
                    </NavGroup>
                    <NavGroup title="Layouts">
                        <NavRouterItem<AppRoute> to=AppRoute::Layout(Layout::Bullseye)>{"Bullseye"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Layout(Layout::Flex)>{"Flex"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Layout(Layout::Gallery)>{"Gallery"}</NavRouterItem<AppRoute>>
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

                            AppRoute::Layout(Layout::Bullseye) => html!{<layouts::BullseyeExample/>},
                            AppRoute::Layout(Layout::Flex) => html!{<layouts::FlexExample/>},
                            AppRoute::Layout(Layout::Gallery) => html!{<layouts::GalleryExample/>},

                            AppRoute::Component(Component::Badge) => html!{<components::BadgeExample/>},
                            AppRoute::Component(Component::Clipboard) => html!{<components::ClipboardExample/>},
                            AppRoute::Component(Component::EmptyState) => html!{<components::EmptyStateExample/>},
                            AppRoute::Component(Component::Form) => html!{<components::FormExample/>},
                            AppRoute::Component(Component::Label) => html!{<components::LabelExample/>},
                            AppRoute::Component(Component::Popover) => html!{<components::PopoverExample/>},
                            AppRoute::Component(Component::Table) => html!{<components::TableExample/>},
                            AppRoute::Component(Component::Tooltip) => html!{<components::TooltipExample/>},
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
