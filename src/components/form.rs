use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

pub struct FormExample {}

impl Component for FormExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let example = example! {"Text Input" =>
            <Form>
                <FormGroup label="Normal">
                    <TextInput/>
                </FormGroup>
            </Form>
        };

        html! {
            <>
                <ExamplePage title="Form">
                    <Form>
                        <FormGroup label="Test">
                            <Button label="Click me" variant={Variant::Primary}/>
                        </FormGroup>
                        <FormGroup label="Test" required=true helper_text="Some help for you.">
                            <Button label="Click me too" variant={Variant::Secondary}/>
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
                            <TextInput state={InputState::Success}/>
                        </FormGroup>
                        <FormGroup label="Warning">
                            <TextInput state={InputState::Warning}/>
                        </FormGroup>
                        <FormGroup label="Error">
                            <TextInput state={InputState::Error}/>
                        </FormGroup>
                        <FormGroup label="Read Only">
                            <TextInput readonly=true/>
                        </FormGroup>
                        <FormGroup label="Disabled">
                            <TextInput disabled=true/>
                        </FormGroup>
                        <FormGroup label="Search">
                            <TextInput icon={TextInputIcon::Search}/>
                        </FormGroup>
                        <FormGroup label="Calendar">
                            <TextInput icon={TextInputIcon::Calendar}/>
                        </FormGroup>
                        <FormGroup label="Clock">
                            <TextInput icon={TextInputIcon::Clock}/>
                        </FormGroup>
                    </Form>
                </ExamplePage>
            </>
        }
    }
}
