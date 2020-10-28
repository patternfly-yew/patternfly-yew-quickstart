use crate::example;

use patternfly_yew::*;
use yew::prelude::*;

pub struct FlexExample {}

impl Component for FlexExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let style = "border: 1px solid;";

        let example1 = example! {"Flex" |
            <Flex>
                <FlexItem><div style=style>{"Flex item #1"}</div></FlexItem>
                <FlexItem><div style=style>{"Flex item #2"}</div></FlexItem>
                <FlexItem><div style=style>{"Flex item #3"}</div></FlexItem>
            </Flex>
        };

        let example2 = example! {"Flex (nested and grow)" |
            <Flex>
                <Flex modifiers=vec![FlexModifier::Grow.all()]>
                    <FlexItem><div style=style>{"Flex item #1"}</div></FlexItem>
                    <FlexItem><div style=style>{"Flex item #2"}</div></FlexItem>
                    <FlexItem><div style=style>{"Flex item #3"}</div></FlexItem>
                </Flex>
                <Flex>
                    <FlexItem><div style=style>{"Flex item #4"}</div></FlexItem>
                    <FlexItem><div style=style>{"Flex item #5"}</div></FlexItem>
                    <FlexItem><div style=style>{"Flex item #6"}</div></FlexItem>
                </Flex>
            </Flex>
        };

        let example3 = example! {"Flex (column on lg)" |
            <Flex>
                <Flex modifiers=vec![FlexModifier::Column.lg()]>
                    <FlexItem><div style=style>{"Flex item #1"}</div></FlexItem>
                    <FlexItem><div style=style>{"Flex item #2"}</div></FlexItem>
                    <FlexItem><div style=style>{"Flex item #3"}</div></FlexItem>
                </Flex>
            </Flex>
        };

        html! {
            <>
                <PageSection variant=PageSectionVariant::Light limit_width=true>
                    <Content>
                        <h1>{"Flex Layout"}</h1>
                    </Content>
                </PageSection>
                <PageSection>
                    <Content>
                        {example1}
                        {example2}
                        {example3}
                    </Content>
                </PageSection>
            </>
        }
    }
}
