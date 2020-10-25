use patternfly_yew::*;
use yew::prelude::*;

pub struct FormExample {}

impl Component for FormExample {
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
                        <h1>{"Form"}</h1>
                    </Content>
                </PageSection>
                <PageSection>
                    <Content>
                        <Form>
                            <FormGroup label="Test">
                                <Button label="Click me"/>
                            </FormGroup>
                            <FormGroup label="Test" required=true helper_text="Some help for you.">
                                <Button label="Click me too"/>
                            </FormGroup>
                        </Form>
                    </Content>
                </PageSection>
            </>
        }
    }
}
