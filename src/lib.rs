#![recursion_limit = "1024"]

mod components;
mod counter;
mod example;
mod full;
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
    #[to = "/alert"]
    Alert,
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
pub enum FullPage {
    #[to = "/login"]
    Login,
}

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/components{*:rest}"]
    Component(Component),
    #[to = "/fullpage{*:rest}"]
    FullPageExample(FullPage),
    #[to = "/full{*:rest}"]
    FullPage(FullPage),
    #[to = "/layout{*:rest}"]
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
        html! {
            <>
            <ToastViewer/>
            <Router<AppRoute, ()>
                redirect = Router::redirect(|_|AppRoute::Index)
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Counter => Self::page(html!{<Counter/>}),
                        AppRoute::Index => Self::page(html!{<Index/>}),

                        AppRoute::FullPageExample(FullPage::Login) => Self::page(html!{<full::FullPageExample url="../../full/login"/>}),
                        AppRoute::FullPage(FullPage::Login) => html!{<full::LoginPageExample/>},

                        AppRoute::Layout(Layout::Bullseye) => Self::page(html!{<layouts::BullseyeExample/>}),
                        AppRoute::Layout(Layout::Flex) => Self::page(html!{<layouts::FlexExample/>}),
                        AppRoute::Layout(Layout::Gallery) => Self::page(html!{<layouts::GalleryExample/>}),

                        AppRoute::Component(Component::Alert) => Self::page(html!{<components::AlertExample/>}),
                        AppRoute::Component(Component::Badge) => Self::page(html!{<components::BadgeExample/>}),
                        AppRoute::Component(Component::Clipboard) => Self::page(html!{<components::ClipboardExample/>}),
                        AppRoute::Component(Component::EmptyState) => Self::page(html!{<components::EmptyStateExample/>}),
                        AppRoute::Component(Component::Form) => Self::page(html!{<components::FormExample/>}),
                        AppRoute::Component(Component::Label) => Self::page(html!{<components::LabelExample/>}),
                        AppRoute::Component(Component::Popover) => Self::page(html!{<components::PopoverExample/>}),
                        AppRoute::Component(Component::Table) => Self::page(html!{<components::TableExample/>}),
                        AppRoute::Component(Component::Tooltip) => Self::page(html!{<components::TooltipExample/>}),
                    }
                })
            />
            </>
        }
    }
}

impl Model {
    fn page(html: Html) -> Html {
        let sidebar = html_nested! {
            <PageSidebar>
                <Nav>
                    <NavGroup title="Basics">
                        <NavRouterItem<AppRoute> to=AppRoute::Index>{"Index"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Counter>{"Counter"}</NavRouterItem<AppRoute>>
                    </NavGroup>
                    <NavGroup title="Components">
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Alert)>{"Alert"}</NavRouterItem<AppRoute>>
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
                    <NavGroup title="Full Page">
                        <NavRouterItem<AppRoute> to=AppRoute::FullPageExample(FullPage::Login)>{"Login Page"}</NavRouterItem<AppRoute>>
                    </NavGroup>
                </Nav>
            </PageSidebar>
        };

        html! {
            <Page
                logo={html_nested!{
                    <Logo src="https://www.patternfly.org/assets/images/PF-Masthead-Logo.svg" alt="Patternfly Logo" />
                }}
                sidebar=sidebar
                >
                { html }
            </Page>
        }
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
    Ok(())
}
