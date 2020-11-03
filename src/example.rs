use yew::prelude::*;

use crate::{Level, PageSection, PageSectionVariant, Size, Title};

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
                                            {"html! {\n"}
                                                {&code}
                                            {"\n}"}
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

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub children: Children,
}

pub struct Example {
    props: Props,
}

impl Component for Example {
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
