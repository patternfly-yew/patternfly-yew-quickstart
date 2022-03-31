use crate::{example::ExamplePage, example2};

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
        let example = example2! {"Text Input" => "form.1.example" };

        html! {
            <>
                <ExamplePage title="Form">
                    <Form>
                        <FormGroup label="Test">
                            <Button label="Click me" variant={Variant::Primary}/>
                        </FormGroup>
                        <FormGroup label="Test" required=true helper_text={Some("Some help for you.".into())}>
                            <Button label="Click me too" variant={Variant::Secondary}/>
                        </FormGroup>
                    </Form>

                    {
                        example
                    }

                    <Title level={Level::H2}>{"Form Validation"}</Title>

                    { Self::form_validation() }

                    <Title level={Level::H2}>{"More"}</Title>

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

impl FormExample {
    pub fn form_validation() -> Html {
        html!(
            <Form>
                <FormGroupValidated<TextInput>
                    label="Some text"
                    required=true
                    validator={Validator::from(|ctx: ValidationContext<String>| {
                        if ctx.value.is_empty() {
                            ValidationResult::error("Must not be empty")
                        } else if ctx.value.contains("foo") {
                            ValidationResult::warning("Should not contain 'foo'")
                        } else if ctx.value == "Nur ein Wort" {
                            ValidationResult::new(InputState::Success, "Congratulations, you found the magic value!")
                        } else {
                            ValidationResult::default()
                        }
                    })}
                >
                    <TextInput
                        placeholder="Enter some text"
                    />
                </FormGroupValidated<TextInput>>
            </Form>
        )
    }
}
