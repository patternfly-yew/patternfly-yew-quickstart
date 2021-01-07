use yew::prelude::*;

use crate::{
    Flex, FlexItem, FlexModifier, Level, PageSection, PageSectionVariant, Size, Title,
    WithBreakpointExt,
};

#[macro_export]
macro_rules! example {
    ($title:expr => $($t:tt)*) => {
        {
        let code = {stringify!($($t)*)};
        html! {
            <>
                <Flex modifiers=vec![FlexModifier::Column.all()]>

                    <FlexItem><Title level=Level::H2 size=Size::XLarge>{$title}</Title></FlexItem>

                    <Flex modifiers=vec![FlexModifier::Column.all()]>

                        <FlexItem>
                            <Title level=Level::H3 size=Size::Large>{"Example"}</Title>
                            $($t)*
                        </FlexItem>

                        <FlexItem>
                            <Title level=Level::H3 size=Size::Large>{"Code"}</Title>

                            <div class="pf-c-code-editor">
                                <div class="pf-c-code-editor__main">
                                    <div class="pf-c-code-editor__code">
                                        <pre class="pf-c-code-editor__code-pre">
                                            {&code}
                                        </pre>
                                    </div>
                                </div>
                            </div>

                        </FlexItem>
                    </Flex>

                </Flex>
            </>
        }
        }
    };
}

#[macro_export]
macro_rules! example2 {
    ($title:expr => $file:expr) => {{
        html! {
            <>
                <Example title=$title code=include_str!($file)>{include!($file)}</Example>
            </>
        }
    }};
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub children: Children,
}

pub struct ExamplePage {
    props: Props,
}

impl Component for ExamplePage {
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
        html! {
            <>
                <PageSection variant=PageSectionVariant::Light limit_width=true>
                    <Title level=Level::H1 size=Size::XXLarge>{ &self.props.title }</Title>
                </PageSection>
                { for self.props.children.iter().map(|child|{
                    html!{<PageSection>{child}</PageSection>}
                }) }
            </>
        }
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ExampleProps {
    pub title: String,
    pub children: Children,
    pub code: String,
}

pub struct Example {
    props: ExampleProps,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

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
        html! {
            <>
                <Flex modifiers=vec![FlexModifier::Column.all()]>

                    <FlexItem><Title level=Level::H2 size=Size::XLarge> { &self.props.title } </Title></FlexItem>

                    <Flex modifiers=vec![FlexModifier::Column.all()]>

                        <FlexItem>
                            <Title level=Level::H3 size=Size::Large>{"Example"}</Title>
                            { for self.props.children.iter() }
                        </FlexItem>

                        <FlexItem>
                            <Title level=Level::H3 size=Size::Large>{"Code"}</Title>

                            <div class="pf-c-code-editor">
                                <div class="pf-c-code-editor__main">
                                    <div class="pf-c-code-editor__code">
                                        <pre class="pf-c-code-editor__code-pre">
                                            {&self.props.code}
                                        </pre>
                                    </div>
                                </div>
                            </div>

                        </FlexItem>
                    </Flex>

                </Flex>
            </>
        }
    }
}
