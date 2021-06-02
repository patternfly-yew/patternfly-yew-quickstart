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
    #[to = "/applauncher"]
    AppLauncher,
    #[to = "/badge"]
    Badge,
    #[to = "/clipboard"]
    Clipboard,
    #[to = "/context_selector"]
    ContextSelector,
    #[to = "/dropdown"]
    Dropdown,
    #[to = "/empty"]
    EmptyState,
    #[to = "/form"]
    Form,
    #[to = "/label"]
    Label,
    #[to = "/popover"]
    Popover,
    #[to = "/switch"]
    Switch,
    #[to = "/tabs{*}"]
    Tabs(components::TabRoutes),
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
    #[to = "/grid"]
    Grid,
    #[to = "/split"]
    Split,
    #[to = "/stack"]
    Stack,
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
            <BackdropViewer/>
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
                        AppRoute::Layout(Layout::Grid) => Self::page(html!{<layouts::GridExample/>}),
                        AppRoute::Layout(Layout::Split) => Self::page(html!{<layouts::SplitExample/>}),
                        AppRoute::Layout(Layout::Stack) => Self::page(html!{<layouts::StackExample/>}),

                        AppRoute::Component(Component::Alert) => Self::page(html!{<components::AlertExample/>}),
                        AppRoute::Component(Component::AppLauncher) => Self::page(html!{<components::AppLauncherExample/>}),
                        AppRoute::Component(Component::Badge) => Self::page(html!{<components::BadgeExample/>}),
                        AppRoute::Component(Component::Clipboard) => Self::page(html!{<components::ClipboardExample/>}),
                        AppRoute::Component(Component::ContextSelector) => Self::page(html!{<components::ContextSelectorExample/>}),
                        AppRoute::Component(Component::Dropdown) => Self::page(html!{<components::DropdownExample/>}),
                        AppRoute::Component(Component::EmptyState) => Self::page(html!{<components::EmptyStateExample/>}),
                        AppRoute::Component(Component::Form) => Self::page(html!{<components::FormExample/>}),
                        AppRoute::Component(Component::Label) => Self::page(html!{<components::LabelExample/>}),
                        AppRoute::Component(Component::Popover) => Self::page(html!{<components::PopoverExample/>}),
                        AppRoute::Component(Component::Switch) => Self::page(html!{<components::SwitchExample/>}),
                        AppRoute::Component(Component::Table) => Self::page(html!{<components::TableExample/>}),
                        AppRoute::Component(Component::Tabs(current)) => Self::page(html!{<components::TabsExample current=current/>}),
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
                    <NavRouterExpandable<AppRoute> title="Basics">
                        <NavRouterItem<AppRoute> to=AppRoute::Index>{"Index"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Counter>{"Counter"}</NavRouterItem<AppRoute>>
                        <NavItem external=true to="https://github.com/ctron/patternfly-yew">{"PatternFly Yew"}</NavItem>
                    </NavRouterExpandable<AppRoute>>
                    <NavRouterExpandable<AppRoute> title="Components">
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Alert)>{"Alert"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::AppLauncher)>{"AppLauncher"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Badge)>{"Badge"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Clipboard)>{"Clipboard"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::ContextSelector)>{"ContextSelector"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Dropdown)>{"Dropdown"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::EmptyState)>{"Empty state"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Form)>{"Form"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Label)>{"Label"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Popover)>{"Popover"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Switch)>{"Switch"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Table)>{"Table"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Tabs(components::TabRoutes::Foo))>{"Tabs"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Component(Component::Tooltip)>{"Tooltip"}</NavRouterItem<AppRoute>>
                    </NavRouterExpandable<AppRoute>>
                    <NavRouterExpandable<AppRoute> title="Layouts">
                        <NavRouterItem<AppRoute> to=AppRoute::Layout(Layout::Bullseye)>{"Bullseye"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Layout(Layout::Flex)>{"Flex"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Layout(Layout::Gallery)>{"Gallery"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Layout(Layout::Grid)>{"Grid"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Layout(Layout::Split)>{"Split"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to=AppRoute::Layout(Layout::Stack)>{"Stack"}</NavRouterItem<AppRoute>>
                    </NavRouterExpandable<AppRoute>>
                    <NavRouterExpandable<AppRoute> title="Full Page">
                        <NavRouterItem<AppRoute> to=AppRoute::FullPageExample(FullPage::Login)>{"Login Page"}</NavRouterItem<AppRoute>>
                    </NavRouterExpandable<AppRoute>>
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
