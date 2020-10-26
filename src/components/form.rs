use crate::example;
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
        let example = example! {"Text Input" |
            <Form>
                <FormGroup label="Normal">
                    <TextInput/>
                </FormGroup>
            </Form>
        };

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

                        {
                            example
                        }

                        <h2>{"More"}</h2>

                        <Form>
                            <FormGroup label="Normal">
                                <TextInput/>
                            </FormGroup>
                            <FormGroup label="Success">
                                <TextInput state=InputState::Success/>
                            </FormGroup>
                            <FormGroup label="Warning">
                                <TextInput state=InputState::Warning/>
                            </FormGroup>
                            <FormGroup label="Error">
                                <TextInput state=InputState::Error/>
                            </FormGroup>
                            <FormGroup label="Read Only">
                                <TextInput readonly=true/>
                            </FormGroup>
                            <FormGroup label="Disabled">
                                <TextInput disabled=true/>
                            </FormGroup>
                            <FormGroup label="Search">
                                <TextInput icon=TextInputIcon::Search/>
                            </FormGroup>
                            <FormGroup label="Calendar">
                                <TextInput icon=TextInputIcon::Calendar/>
                            </FormGroup>
                            <FormGroup label="Clock">
                                <TextInput icon=TextInputIcon::Clock/>
                            </FormGroup>
                        </Form>
                    </Content>
                </PageSection>
            </>
        }
    }
}
