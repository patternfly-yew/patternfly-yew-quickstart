use crate::example;

use patternfly_yew::*;
use yew::prelude::*;

pub struct ClipboardExample {}

impl Component for ClipboardExample {
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
        let example1 = example! {"Clipboard" |
            <Clipboard value="Foo bar"/>
        };

        html! {
            <>
                <PageSection variant=PageSectionVariant::Light limit_width=true>
                    <Content>
                        <h1>{"Clipboard"}</h1>
                    </Content>
                </PageSection>
                <PageSection>
                    <Content>
                        {example1}
                    </Content>
                </PageSection>
            </>
        }
    }
}
