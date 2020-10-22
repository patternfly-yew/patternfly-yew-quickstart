use yew::prelude::*;
use yew_router::prelude::*;
use yew::prelude::*;

use patternfly_yew::*;

pub struct Index {
}


impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties,_link: ComponentLink<Self>) -> Self {
        Self {  }
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
                        <h1>{"Patternfly Yew Quickstart"}</h1>
                    </Content>
                </PageSection>
                <PageSection>
                    <Content>
                        {"Pick an example on the left"}
                    </Content>
                </PageSection>
            </>
        }
    }
}