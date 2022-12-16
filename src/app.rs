use crate::components;
use crate::counter::*;
use crate::full;
use crate::index::*;
use crate::layouts;

use patternfly_yew::*;

use yew::prelude::*;
use yew_nested_router::prelude::{Switch as RouterSwitch, *};
use yew_nested_router::Target;

#[derive(Debug, Clone, PartialEq, Eq, Target)]
pub enum Component {
    Alert,
    AppLauncher,
    Backdrop,
    Badge,
    Button,
    Clipboard,
    Chip,
    #[target(rename = "context_selector")]
    ContextSelector,
    Dropdown,
    #[target(rename = "empty")]
    EmptyState,
    Form,
    Label,
    Modal,
    Popover,
    Select,
    Slider,
    Spinner,
    Switch,
    Tabs(components::TabRoutes),
    Table,
    Text,
    Title,
    Toast,
    Tooltip,
}

#[derive(Debug, Clone, PartialEq, Eq, Target)]
pub enum Layout {
    Bullseye,
    Flex,
    Gallery,
    Grid,
    Split,
    Stack,
}

#[derive(Debug, Clone, PartialEq, Eq, Target)]
pub enum FullPage {
    Login,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AppRoute {
    Component(Component),
    #[target(rename = "fullpage")]
    FullPageExample(FullPage),
    #[target(rename = "full")]
    FullPage(FullPage),
    Layout(Layout),
    Counter,
    #[default]
    Index,
}

#[function_component(Application)]
pub fn app() -> Html {
    html! {
        <>
        <BackdropViewer>
            <ToastViewer>
                <Router<AppRoute> default={AppRoute::Index}>
                    <RouterSwitch<AppRoute> render={switch_app_route} />
                </Router<AppRoute>>
            </ToastViewer>
        </BackdropViewer>
        </>
    }
}

fn switch_app_route(target: AppRoute) -> Html {
    let component = |target: Component| match target {
        Component::Alert => html! {<components::AlertExample/>},
        Component::AppLauncher => html! {<components::AppLauncherExample/>},
        Component::Backdrop => html! {<components::BackdropExample/>},
        Component::Badge => html! {<components::BadgeExample/>},
        Component::Button => html! {<components::ButtonExample/>},
        Component::Chip => html! {<components::ChipExample/>},
        Component::Clipboard => html! {<components::ClipboardExample/>},
        Component::ContextSelector => html! {<components::ContextSelectorExample/>},
        Component::Dropdown => html! {<components::DropdownExample/>},
        Component::EmptyState => html! {<components::EmptyStateExample/>},
        Component::Form => html! {<components::FormExample/>},
        Component::Label => html! {<components::LabelExample/>},
        Component::Modal => html! {<components::ModalExample/>},
        Component::Popover => html! {<components::PopoverExample/>},
        Component::Select => html! {<components::SelectExample/>},
        Component::Slider => html! {<components::SliderExample/>},
        Component::Spinner => html! {<components::SpinnerExample/>},
        Component::Switch => html! {<components::SwitchExample/>},
        Component::Table => html! {<components::TableExample/>},
        Component::Tabs(current) => html! {<components::TabsExample current={current}/>},
        Component::Text => html! {<components::TextExample/>},
        Component::Title => html! {<components::TitleExample/>},
        Component::Toast => html! {<components::ToastExample/>},
        Component::Tooltip => html! {<components::TooltipExample/>},
    };

    let layout = |target: Layout| match target {
        Layout::Bullseye => html! {<layouts::BullseyeExample/>},
        Layout::Flex => html! {<layouts::FlexExample/>},
        Layout::Gallery => html! {<layouts::GalleryExample/>},
        Layout::Grid => html! {<layouts::GridExample/>},
        Layout::Split => html! {<layouts::SplitExample/>},
        Layout::Stack => html! {<layouts::StackExample/>},
    };

    let fullpage = |target: FullPage| match target {
        FullPage::Login => html! {<full::LoginPageExample/>},
    };

    let fullpage_entrypoint = |target: FullPage| match target {
        FullPage::Login => {
            html! {<full::FullPageExample url="../../full/login"/>}
        }
    };

    match target {
        AppRoute::Counter => html! {<AppPage><Counter/></AppPage>},
        AppRoute::Index => html! {<AppPage><Index/></AppPage>},

        AppRoute::FullPageExample(_) => {
            html!(
                <AppPage>
                    <Scope<AppRoute, FullPage> mapper={AppRoute::mapper_full_page_example}>
                        <RouterSwitch<FullPage> render={fullpage_entrypoint}/>
                    </Scope<AppRoute, FullPage>>
                </AppPage>
            )
        }
        AppRoute::FullPage(_) => {
            html!(
                <Scope<AppRoute, FullPage> mapper={AppRoute::mapper_full_page}>
                    <RouterSwitch<FullPage> render={fullpage}/>
                </Scope<AppRoute, FullPage>>
            )
        }

        AppRoute::Layout(_) => {
            html!(
                <AppPage>
                    <Scope<AppRoute, Layout>  mapper={AppRoute::mapper_layout}>
                        <RouterSwitch<Layout> render={layout}/>
                    </Scope<AppRoute, Layout>>
                </AppPage>
            )
        }
        AppRoute::Component(_) => {
            html!(
                <AppPage>
                    <Scope<AppRoute, Component> mapper={AppRoute::mapper_component}>
                        <RouterSwitch<Component> render={component}/>
                    </Scope<AppRoute, Component>>
                </AppPage>
            )
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PageProps {
    pub children: Children,
}

#[function_component(AppPage)]
fn page(props: &PageProps) -> Html {
    let sidebar = html_nested! {
        <PageSidebar>
            <Nav>
                <NavExpandable title="Basics">
                    <NavRouterItem<AppRoute> to={AppRoute::Index}>{"Index"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Counter}>{"Counter"}</NavRouterItem<AppRoute>>
                    <NavItem external=true to="https://github.com/ctron/patternfly-yew">{"PatternFly Yew"}</NavItem>
                </NavExpandable>
                <NavExpandable title="Components">
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Alert)}>{"Alert"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::AppLauncher)}>{"AppLauncher"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Backdrop)}>{"Backdrop"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Badge)}>{"Badge"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Button)}>{"Button"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Chip)}>{"Chip"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Clipboard)}>{"Clipboard"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::ContextSelector)}>{"ContextSelector"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Dropdown)}>{"Dropdown"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::EmptyState)}>{"Empty state"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Form)}>{"Form"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Label)}>{"Label"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Modal)}>{"Modal"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Popover)}>{"Popover"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Select)}>{"Select"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Slider)}>{"Slider"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Spinner)}>{"Spinner"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Switch)}>{"Switch"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Table)}>{"Table"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Tabs(components::TabRoutes::Foo))} predicate={AppRoute::with_component(Component::is_tabs)}>{"Tabs"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Text)}>{"Text"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Title)}>{"Title"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Toast)}>{"Toast"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Tooltip)}>{"Tooltip"}</NavRouterItem<AppRoute>>
                </NavExpandable>
                <NavExpandable title="Layouts">
                    <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Bullseye)}>{"Bullseye"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Flex)}>{"Flex"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Gallery)}>{"Gallery"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Grid)}>{"Grid"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Split)}>{"Split"}</NavRouterItem<AppRoute>>
                    <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Stack)}>{"Stack"}</NavRouterItem<AppRoute>>
                </NavExpandable>
                <NavExpandable title="Full Page">
                    <NavRouterItem<AppRoute> to={AppRoute::FullPageExample(FullPage::Login)}>{"Login Page"}</NavRouterItem<AppRoute>>
                </NavExpandable>
            </Nav>
        </PageSidebar>
    };

    let logo = html! {
        <Logo src="https://www.patternfly.org/assets/images/PF-Masthead-Logo.svg" alt="Patternfly Logo" />
    };

    html! {
        <Page
            logo={logo}
            sidebar={sidebar}
            >
            { for props.children.iter() }
        </Page>
    }
}
