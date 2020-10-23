use patternfly_yew::*;
use yew::prelude::*;

pub struct BadgeExample {}

impl Component for BadgeExample {
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
        html! {
            <>
                <PageSection variant=PageSectionVariant::Light limit_width=true>
                    <Content>
                        <h1>{"Badge"}</h1>
                    </Content>
                </PageSection>
                <PageSection>
                    <Content>
                        <Badge>{"123"}</Badge>
                        <Badge read=true>{"123"}</Badge>
                    </Content>
                </PageSection>
            </>
        }
    }
}
