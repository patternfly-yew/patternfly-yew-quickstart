#![recursion_limit = "256"]

use patternfly_yew::*;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let _sidebar = html_nested! {
            <PageSidebar>
            </PageSidebar>
        };
        let _header_tools = html! { {"Foo"} };

        let _logo = html_nested! {
            <Logo src="https://www.patternfly.org/assets/images/PF-Masthead-Logo.svg" alt="Patternfly Logo" />
        };

        html! {
            <Page
                logo={html_nested!{
                    <Logo src="https://www.patternfly.org/assets/images/PF-Masthead-Logo.svg" alt="Patternfly Logo" />
                }}>
                <PageSection variant=PageSectionVariant::Light limit_width=true>
                    <Content>
                        <h1>{"Counting clicks"}</h1>
                    </Content>
                </PageSection>
                <PageSection>
                    <Gallery gutter=true>
                        <Card
                            selectable=true
                            selected=true
                            title={html_nested!{<>
                                {"Clicks"}
                            </>}}
                            >

                            <p>{ self.value }</p>

                        </Card>
                    </Gallery>
                </PageSection>
                <PageSection>
                    <Form>
                        <Button label="Add One" icon=Some(Icon::PlusCircleIcon) variant=Variant::Link onclick=self.link.callback(|_| Msg::AddOne)/>
                    </Form>
                </PageSection>
            </Page>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
