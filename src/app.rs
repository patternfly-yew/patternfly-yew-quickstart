use crate::components;
use crate::counter::*;
use crate::full;
use crate::index::*;
use crate::layouts;

use patternfly_yew::*;

use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::router::Render;

pub struct Model {}

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
    #[to = "/modal"]
    Modal,
    #[to = "/popover"]
    Popover,
    #[to = "/select"]
    Select,
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
    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <>
            <BackdropViewer/>
            <ToastViewer/>

            <Router<AppRoute, ()>
                redirect = {Router::redirect(|_|AppRoute::Index)}
                render = {Self::switch_main()}
            />
            </>
        }
    }
}

impl Model {
    fn switch_main() -> Render<AppRoute, ()> {
        Router::render(|switch: AppRoute| match switch {
            AppRoute::Counter => Self::page(html! {<Counter/>}),
            AppRoute::Index => Self::page(html! {<Index/>}),

            AppRoute::FullPageExample(FullPage::Login) => {
                Self::page(html! {<full::FullPageExample url="../../full/login"/>})
            }
            AppRoute::FullPage(FullPage::Login) => html! {<full::LoginPageExample/>},

            AppRoute::Layout(Layout::Bullseye) => Self::page(html! {<layouts::BullseyeExample/>}),
            AppRoute::Layout(Layout::Flex) => Self::page(html! {<layouts::FlexExample/>}),
            AppRoute::Layout(Layout::Gallery) => Self::page(html! {<layouts::GalleryExample/>}),
            AppRoute::Layout(Layout::Grid) => Self::page(html! {<layouts::GridExample/>}),
            AppRoute::Layout(Layout::Split) => Self::page(html! {<layouts::SplitExample/>}),
            AppRoute::Layout(Layout::Stack) => Self::page(html! {<layouts::StackExample/>}),

            AppRoute::Component(Component::Alert) => {
                Self::page(html! {<components::AlertExample/>})
            }
            AppRoute::Component(Component::AppLauncher) => {
                Self::page(html! {<components::AppLauncherExample/>})
            }
            AppRoute::Component(Component::Badge) => {
                Self::page(html! {<components::BadgeExample/>})
            }
            AppRoute::Component(Component::Clipboard) => {
                Self::page(html! {<components::ClipboardExample/>})
            }
            AppRoute::Component(Component::ContextSelector) => {
                Self::page(html! {<components::ContextSelectorExample/>})
            }
            AppRoute::Component(Component::Dropdown) => {
                Self::page(html! {<components::DropdownExample/>})
            }
            AppRoute::Component(Component::EmptyState) => {
                Self::page(html! {<components::EmptyStateExample/>})
            }
            AppRoute::Component(Component::Form) => Self::page(html! {<components::FormExample/>}),
            AppRoute::Component(Component::Label) => {
                Self::page(html! {<components::LabelExample/>})
            }
            AppRoute::Component(Component::Modal) => {
                Self::page(html! {<components::ModalExample/>})
            }
            AppRoute::Component(Component::Popover) => {
                Self::page(html! {<components::PopoverExample/>})
            }
            AppRoute::Component(Component::Select) => {
                Self::page(html! {<components::SelectExample/>})
            }
            AppRoute::Component(Component::Switch) => {
                Self::page(html! {<components::SwitchExample/>})
            }
            AppRoute::Component(Component::Table) => {
                Self::page(html! {<components::TableExample/>})
            }
            AppRoute::Component(Component::Tabs(current)) => {
                Self::page(html! {<components::TabsExample current={current}/>})
            }
            AppRoute::Component(Component::Tooltip) => {
                Self::page(html! {<components::TooltipExample/>})
            }
        })
    }

    fn page(html: Html) -> Html {
        let sidebar = html_nested! {
            <PageSidebar>
                <Nav>
                    <NavRouterExpandable<AppRoute> title="Basics">
                        <NavRouterItem<AppRoute> to={AppRoute::Index}>{"Index"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Counter}>{"Counter"}</NavRouterItem<AppRoute>>
                        <NavItem external=true to="https://github.com/ctron/patternfly-yew">{"PatternFly Yew"}</NavItem>
                    </NavRouterExpandable<AppRoute>>
                    <NavRouterExpandable<AppRoute> title="Components">
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Alert)}>{"Alert"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::AppLauncher)}>{"AppLauncher"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Badge)}>{"Badge"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Clipboard)}>{"Clipboard"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::ContextSelector)}>{"ContextSelector"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Dropdown)}>{"Dropdown"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::EmptyState)}>{"Empty state"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Form)}>{"Form"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Label)}>{"Label"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Modal)}>{"Modal"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Popover)}>{"Popover"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Select)}>{"Select"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Switch)}>{"Switch"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Table)}>{"Table"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Tabs(components::TabRoutes::Foo))}>{"Tabs"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Tooltip)}>{"Tooltip"}</NavRouterItem<AppRoute>>
                    </NavRouterExpandable<AppRoute>>
                    <NavRouterExpandable<AppRoute> title="Layouts">
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Bullseye)}>{"Bullseye"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Flex)}>{"Flex"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Gallery)}>{"Gallery"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Grid)}>{"Grid"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Split)}>{"Split"}</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Stack)}>{"Stack"}</NavRouterItem<AppRoute>>
                    </NavRouterExpandable<AppRoute>>
                    <NavRouterExpandable<AppRoute> title="Full Page">
                        <NavRouterItem<AppRoute> to={AppRoute::FullPageExample(FullPage::Login)}>{"Login Page"}</NavRouterItem<AppRoute>>
                    </NavRouterExpandable<AppRoute>>
                </Nav>
            </PageSidebar>
        };

        let logo = html_nested! {
            <Logo src="https://www.patternfly.org/assets/images/PF-Masthead-Logo.svg" alt="Patternfly Logo" />
        };

        html! {
            <Page
                logo={logo}
                sidebar={sidebar}
                >
                { html }
            </Page>
        }
    }
}
