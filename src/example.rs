use patternfly_yew::prelude::*;
use yew::prelude::*;

#[macro_export]
macro_rules! example {
    ($title:expr => $($t:tt)*) => {
        {
        let code = {stringify!($($t)*)};
        html! {
            <>
                <Flex modifiers={[FlexModifier::Column.all()]}>

                <FlexItem><Title level={Level::H2} size={Size::XLarge}>{$title}</Title></FlexItem>

                    <Flex modifiers={[FlexModifier::Column.all()]}>

                        <FlexItem>
                            $($t)*
                        </FlexItem>

                        <FlexItem>
                            <Title level={Level::H3} size={Size::Large}>{"Code"}</Title>

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

/// Include an example from an external file.
///
/// ```rust
/// # use yew::prelude::*;
/// fn example() -> Html {
///     let example1 = example2!{ "Example" => "file.example" };
///     html!{
///         <div>{example1}</div>
///     }
/// }
/// ```
#[macro_export]
macro_rules! example2 {
    ($title:expr => $file:expr) => {{
        html! {
            <>
                <$crate::example::Example title={$title} code={include_str!($file)}>{{include!($file)}}</$crate::example::Example>
            </>
        }
    }};
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    #[prop_or_default]
    pub subtitle: Children,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ExamplePage)]
pub fn example_page(props: &Props) -> Html {
    html! (
        <>
            <PageSection
                variant={PageSectionVariant::Light}
                limit_width=true
                sticky={[PageSectionSticky::Top]}
            >
                <Content>
                    <Title size={Size::XXXXLarge}>
                        { &props.title }
                    </Title>
                    { for props.subtitle.iter() }
                </Content>
            </PageSection>
            { for props.children.iter().map(|child|{
                html!{<PageSection>{child}</PageSection>}
            }) }
        </>
    )
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ExampleProps {
    pub title: String,
    pub children: Children,
    pub code: String,
}

pub struct Example {}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Flex modifiers={[FlexModifier::Column.all()]}>

                    <FlexItem>
                        <Title level={Level::H2} size={Size::XXLarge}>
                            { &ctx.props().title }
                        </Title>
                    </FlexItem>

                    <Flex modifiers={[FlexModifier::Column.all()]}>

                        <FlexItem>
                            <Title level={Level::H3} size={Size::Large}>{"Example"}</Title>
                            { for ctx.props().children.iter() }
                        </FlexItem>

                        <FlexItem>
                            <Title level={Level::H3} size={Size::Large}>{"Code"}</Title>

                            <div class="pf-c-code-editor">
                                <div class="pf-c-code-editor__main">
                                    <div class="pf-c-code-editor__code">
                                        <pre class="pf-c-code-editor__code-pre">
                                            {&ctx.props().code}
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
