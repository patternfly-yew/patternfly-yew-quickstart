use crate::components;
use crate::counter::*;
use crate::full;
use crate::hook::use_open;
use crate::index::*;
use crate::layouts;
use crate::{icons::Icons, panic::Panic};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::Target;
use yew_nested_router::prelude::{Switch as RouterSwitch, *};

mod about;

#[derive(Debug, Clone, PartialEq, Eq, Target)]
pub enum Component {
    Accordion,
    Alert,
    Avatar,
    Backdrop,
    Badge,
    Brand,
    Breadcrumb,
    Button,
    Card,
    Clipboard,
    CodeBlock,
    Date(Date),
    DescriptionList,
    Divider,
    Drawer,
    Dropdown,
    DualListSelector,
    #[target(rename = "empty")]
    EmptyState,
    ExpandableSection,
    FileUpload,
    Form(Form),
    HelperText,
    Hint,
    Label,
    List,
    Menu(Menu),
    Modal,
    NumberInput,
    Pagination,
    Popover,
    Progress,
    SearchInput,
    SimpleList,
    Skeleton,
    Slider,
    Spinner,
    Switch,
    Tabs(components::TabRoutes),
    Table,
    Text,
    TextInputGroup,
    Title,
    Toast,
    ToggleGroup,
    Tooltip,
    Tree,
    Truncate,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum Form {
    #[default]
    #[target(index)]
    Index,
    Checkbox,
    Radio,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum Menu {
    #[default]
    #[target(index)]
    Index,
    Select,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Target)]
pub enum Date {
    #[default]
    #[target(index)]
    Calendar,
    DatePicker,
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
    Icons,
    Panic,
}

#[function_component(Application)]
pub fn app() -> Html {
    html! {
        <BackdropViewer>
            <ToastViewer>
                <Router<AppRoute> default={AppRoute::Index}>
                    <RouterSwitch<AppRoute> render={switch_app_route} />
                </Router<AppRoute>>
            </ToastViewer>
        </BackdropViewer>
    }
}

fn switch_app_route(target: AppRoute) -> Html {
    let component = |target: Component| match target {
        Component::Accordion => html! { <components::AccordionExample /> },
        Component::Alert => html! { <components::AlertExample /> },
        Component::Avatar => html! { <components::AvatarExample /> },
        Component::Backdrop => html! { <components::BackdropExample /> },
        Component::Badge => html! { <components::BadgeExample /> },
        Component::Brand => html! { <components::BrandExample /> },
        Component::Breadcrumb => html! { <components::BreadcrumbExample /> },
        Component::Button => html! { <components::ButtonExample /> },
        Component::Card => html! { <components::CardExample /> },
        Component::Clipboard => html! { <components::ClipboardExample /> },
        Component::CodeBlock => html! { <components::CodeBlockExample /> },
        Component::Date(Date::Calendar) => html! { <components::CalendarExample /> },
        Component::Date(Date::DatePicker) => html! { <components::DatePickerExample /> },
        Component::DescriptionList => html! { <components::DescriptionListExample /> },
        Component::Divider => html! { <components::DividerExample /> },
        Component::Drawer => html! { <components::DrawerExample /> },
        Component::Dropdown => html! { <components::DropdownExample /> },
        Component::DualListSelector => html! { <components::DualListSelectorExample /> },
        Component::EmptyState => html! { <components::EmptyStateExample /> },
        Component::ExpandableSection => html! { <components::ExpandableSectionExample /> },
        Component::FileUpload => html! { <components::FileUploadExample /> },
        Component::Form(Form::Index) => html! { <components::FormCommonExample /> },
        Component::Form(Form::Checkbox) => html! { <components::FormCheckboxExample /> },
        Component::Form(Form::Radio) => html! { <components::FormRadioExample /> },
        Component::HelperText => html! { <components::HelperTextExample /> },
        Component::Hint => html! { <components::HintExample /> },
        Component::Label => html! { <components::LabelExample /> },
        Component::List => html! { <components::ListExample /> },
        Component::Menu(Menu::Index) => html! { <components::MenuExample /> },
        Component::Menu(Menu::Select) => html! { <components::SimpleSelectExample /> },
        Component::Modal => html! { <components::ModalExample /> },
        Component::NumberInput => html! { <components::NumberExample /> },
        Component::Pagination => html! { <components::PaginationExample /> },
        Component::Popover => html! { <components::PopoverExample /> },
        Component::Progress => html! { <components::ProgressExample /> },
        Component::SearchInput => html! { <components::SearchInputExample /> },
        Component::SimpleList => html! { <components::SimpleListExample /> },
        Component::Skeleton => html! { <components::SkeletonExample /> },
        Component::Slider => html! { <components::SliderExample /> },
        Component::Spinner => html! { <components::SpinnerExample /> },
        Component::Switch => html! { <components::SwitchExample /> },
        Component::Table => html! { <components::TableExample /> },
        Component::Tabs(current) => html! { <components::TabsExample current={current} /> },
        Component::Text => html! { <components::TextExample /> },
        Component::TextInputGroup => html! { <components::TextInputGroupExample /> },
        Component::Title => html! { <components::TitleExample /> },
        Component::Toast => html! { <components::ToastExample /> },
        Component::ToggleGroup => html! { <components::ToggleGroupExample /> },
        Component::Tooltip => html! { <components::TooltipExample /> },
        Component::Tree => html! { <components::TreeExample /> },
        Component::Truncate => html! { <components::TruncateExample /> },
    };

    let layout = |target: Layout| match target {
        Layout::Bullseye => html! { <layouts::BullseyeExample /> },
        Layout::Flex => html! { <layouts::FlexExample /> },
        Layout::Gallery => html! { <layouts::GalleryExample /> },
        Layout::Grid => html! { <layouts::GridExample /> },
        Layout::Split => html! { <layouts::SplitExample /> },
        Layout::Stack => html! { <layouts::StackExample /> },
    };

    let fullpage = |target: FullPage| match target {
        FullPage::Login => html! { <full::LoginPageExample /> },
    };

    let fullpage_entrypoint = |target: FullPage| match target {
        FullPage::Login => {
            html! { <full::FullPageExample url="full/login" /> }
        }
    };

    match target {
        AppRoute::Counter => html! {
            <AppPage>
                <Counter />
            </AppPage>
        },
        AppRoute::Index => html! {
            <AppPage>
                <Index />
            </AppPage>
        },
        AppRoute::Icons => html! {
            <AppPage>
                <Icons />
            </AppPage>
        },
        AppRoute::Panic => html! {
            <AppPage>
                <Panic />
            </AppPage>
        },

        AppRoute::FullPageExample(_) => {
            html!(
                <AppPage>
                    <Scope<AppRoute, FullPage> mapper={AppRoute::mapper_full_page_example}>
                        <RouterSwitch<FullPage> render={fullpage_entrypoint} />
                    </Scope<AppRoute, FullPage>>
                </AppPage>
            )
        }
        AppRoute::FullPage(_) => {
            html!(
                <Scope<AppRoute, FullPage> mapper={AppRoute::mapper_full_page}>
                    <RouterSwitch<FullPage> render={fullpage} />
                </Scope<AppRoute, FullPage>>
            )
        }

        AppRoute::Layout(_) => {
            html!(
                <AppPage>
                    <Scope<AppRoute, Layout> mapper={AppRoute::mapper_layout}>
                        <RouterSwitch<Layout> render={layout} />
                    </Scope<AppRoute, Layout>>
                </AppPage>
            )
        }
        AppRoute::Component(_) => {
            html!(
                <AppPage>
                    <Scope<AppRoute, Component> mapper={AppRoute::mapper_component}>
                        <RouterSwitch<Component> render={component} />
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
                <NavList>
                    <NavExpandable title="Basics">
                        <NavRouterItem<AppRoute> to={AppRoute::Index}>
                            { "Index" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Counter}>
                            { "Counter" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Icons}>
                            { "Icons" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Panic}>
                            { "Panic" }
                        </NavRouterItem<AppRoute>>
                        <NavLink
                            href="https://github.com/patternfly-yew/patternfly-yew"
                            target="_blank"
                        >
                            { "PatternFly Yew " }
                            { Icon::ExternalLinkAlt.with_classes(classes!("pf-v6-u-ml-sm", "pf-v6-u-color-200")) }
                        </NavLink>
                    </NavExpandable>
                    <NavExpandable title="Components">
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Accordion)}>
                            { "Accordion" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Alert)}>
                            { "Alert" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Avatar)}>
                            { "Avatar" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Backdrop)}>
                            { "Backdrop" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Badge)}>
                            { "Badge" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Brand)}>
                            { "Brand" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Breadcrumb)}>
                            { "Breadcrumb" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Button)}>
                            { "Button" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Card)}>
                            { "Card" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Clipboard)}>
                            { "Clipboard" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::CodeBlock)}>
                            { "Code Block" }
                        </NavRouterItem<AppRoute>>
                        <NavExpandable title="Date">
                            <NavRouterItem<AppRoute>
                                to={AppRoute::Component(Component::Date(Date::Calendar))}
                            >
                                { "Calendar" }
                            </NavRouterItem<AppRoute>>
                            <NavRouterItem<AppRoute>
                                to={AppRoute::Component(Component::Date(Date::DatePicker))}
                            >
                                { "DatePicker" }
                            </NavRouterItem<AppRoute>>
                        </NavExpandable>
                        <NavRouterItem<AppRoute>
                            to={AppRoute::Component(Component::DescriptionList)}
                        >
                            { "DescriptionList" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Divider)}>
                            { "Divider" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Drawer)}>
                            { "Drawer" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Dropdown)}>
                            { "Dropdown" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute>
                            to={AppRoute::Component(Component::DualListSelector)}
                        >
                            { "Dual List Selector" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::EmptyState)}>
                            { "Empty state" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute>
                            to={AppRoute::Component(Component::ExpandableSection)}
                        >
                            { "Expandable Section" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::FileUpload)}>
                            { "File Upload" }
                        </NavRouterItem<AppRoute>>
                        <NavExpandable title="Forms">
                            <NavRouterItem<AppRoute>
                                to={AppRoute::Component(Component::Form(Form::Index))}
                            >
                                { "Common" }
                            </NavRouterItem<AppRoute>>
                            <NavRouterItem<AppRoute>
                                to={AppRoute::Component(Component::Form(Form::Checkbox))}
                            >
                                { "Checkbox" }
                            </NavRouterItem<AppRoute>>
                            <NavRouterItem<AppRoute>
                                to={AppRoute::Component(Component::Form(Form::Radio))}
                            >
                                { "Radio" }
                            </NavRouterItem<AppRoute>>
                        </NavExpandable>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::HelperText)}>
                            { "HelperText" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Hint)}>
                            { "Hint" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Label)}>
                            { "Label" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::List)}>
                            { "List" }
                        </NavRouterItem<AppRoute>>
                        <NavExpandable title="Menus">
                            <NavRouterItem<AppRoute>
                                to={AppRoute::Component(Component::Menu(Menu::Index))}
                            >
                                { "Menu" }
                            </NavRouterItem<AppRoute>>
                            <NavRouterItem<AppRoute>
                                to={AppRoute::Component(Component::Menu(Menu::Select))}
                            >
                                { "Select" }
                            </NavRouterItem<AppRoute>>
                        </NavExpandable>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Modal)}>
                            { "Modal" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::NumberInput)}>
                            { "Number Input" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Pagination)}>
                            { "Pagination" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Popover)}>
                            { "Popover" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Progress)}>
                            { "Progress" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::SearchInput)}>
                            { "Search Input" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::SimpleList)}>
                            { "Simple List" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Skeleton)}>
                            { "Skeleton" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Slider)}>
                            { "Slider" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Spinner)}>
                            { "Spinner" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Switch)}>
                            { "Switch" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Table)}>
                            { "Table" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute>
                            to={AppRoute::Component(Component::Tabs(components::TabRoutes::Foo))}
                            predicate={AppRoute::with_component(Component::is_tabs)}
                        >
                            { "Tabs" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Text)}>
                            { "Text" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute>
                            to={AppRoute::Component(Component::TextInputGroup)}
                        >
                            { "Text Input Group" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Title)}>
                            { "Title" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Toast)}>
                            { "Toast" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::ToggleGroup)}>
                            { "Toggle Group" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Tooltip)}>
                            { "Tooltip" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Tree)}>
                            { "Tree" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Component(Component::Truncate)}>
                            { "Truncate" }
                        </NavRouterItem<AppRoute>>
                    </NavExpandable>
                    <NavExpandable title="Layouts">
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Bullseye)}>
                            { "Bullseye" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Flex)}>
                            { "Flex" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Gallery)}>
                            { "Gallery" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Grid)}>
                            { "Grid" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Split)}>
                            { "Split" }
                        </NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Layout(Layout::Stack)}>
                            { "Stack" }
                        </NavRouterItem<AppRoute>>
                    </NavExpandable>
                    <NavExpandable title="Full Page">
                        <NavRouterItem<AppRoute> to={AppRoute::FullPageExample(FullPage::Login)}>
                            { "Login Page" }
                        </NavRouterItem<AppRoute>>
                    </NavExpandable>
                </NavList>
            </Nav>
        </PageSidebar>
    };

    let brand = html! (
        <MastheadBrand>
            <div className="show-light">
                <Brand
                    src="/images/pf-logo.svg"
                    alt="Patternfly Logo"
                    style="--pf-v6-c-brand--Height: 36px;"
                />
            </div>
        </MastheadBrand>
    );

    let callback_github = use_open(
        "https://github.com/patternfly-yew/patternfly-yew-quickstart",
        "_blank",
    );

    let backdropper = use_backdrop();

    let onabout = use_callback((), move |_, ()| {
        if let Some(backdropper) = &backdropper {
            backdropper.open(html!(<about::About />));
        }
    });

    // track dark mode state
    let darkmode = use_state_eq(|| {
        gloo_utils::window()
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()
            .map(|m| m.matches())
            .unwrap_or_default()
    });

    // apply dark mode
    use_effect_with(*darkmode, |state| match state {
        true => gloo_utils::document_element().set_class_name("pf-v6-theme-dark"),
        false => gloo_utils::document_element().set_class_name(""),
    });

    // toggle dark mode
    let onthemeswitch = use_callback(darkmode.setter(), |state, setter| setter.set(state));

    let tools = html!(
        <Toolbar full_height=true>
            <ToolbarContent>
                <ToolbarGroup
                    modifiers={ToolbarElementModifier::End.all()}
                    variant={GroupVariant::IconButton}
                >
                    <ToolbarItem>
                        <patternfly_yew::prelude::Switch
                            checked={*darkmode}
                            onchange={onthemeswitch}
                            label="Dark Theme"
                        />
                    </ToolbarItem>
                    <ToolbarItem>
                        <Button
                            variant={ButtonVariant::Plain}
                            icon={Icon::Github}
                            onclick={callback_github}
                        />
                    </ToolbarItem>
                    <ToolbarItem>
                        <Dropdown
                            position={Position::Right}
                            icon={Icon::QuestionCircle}
                            variant={MenuToggleVariant::Plain}
                        >
                            <MenuAction onclick={onabout}>{ "About" }</MenuAction>
                        </Dropdown>
                    </ToolbarItem>
                </ToolbarGroup>
            </ToolbarContent>
        </Toolbar>
    );

    html! (<Page {brand} {sidebar} {tools}>{ for props.children.iter() }</Page>)
}
