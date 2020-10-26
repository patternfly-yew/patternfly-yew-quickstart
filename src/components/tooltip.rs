use crate::example;

use patternfly_yew::*;
use yew::prelude::*;

pub struct TooltipExample {}

impl Component for TooltipExample {
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
        let example1 = example! {"Tooltip Popup" |
            <TooltipPopup orientation=Orientation::Left text="This is just an example tooltip."/>
        };

        let example2 = example! {"Tooltip" |
            <Tooltip text="This is just an example tooltip.">
                <span style="border: 1px solid black;">{"I have a tooltip."}</span>
            </Tooltip>
        };

        html! {
            <>
                <PageSection variant=PageSectionVariant::Light limit_width=true>
                    <Content>
                        <h1>{"Tooltip"}</h1>
                    </Content>
                </PageSection>
                <PageSection>
                    <Content>
                        {example1}
                        {example2}
                    </Content>
                </PageSection>
            </>
        }
    }
}
