use crate::example;

use patternfly_yew::*;
use yew::prelude::*;

pub struct GalleryExample {}

impl Component for GalleryExample {
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
        let example1 = example! {"Gallery" |
            <Gallery>
                <div>{"Item #1"}</div>
                <div>{"Item #2"}</div>
                <div>{"Item #3"}</div>
            </Gallery>
        };

        let example2 = example! {"Gallery (gutter)" |
            <Gallery gutter=true>
                <div>{"Item #1"}</div>
                <div>{"Item #2"}</div>
                <div>{"Item #3"}</div>
            </Gallery>
        };

        html! {
            <>
                <PageSection variant=PageSectionVariant::Light limit_width=true>
                    <Content>
                        <h1>{"Gallery Layout"}</h1>
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
