use patternfly_yew::prelude::*;
use yew::html::ChildrenRenderer;
use yew::prelude::*;

#[function_component(LoginPageExample)]
pub fn login_page_example() -> Html {
    let header = html! { <>{ "Header" }</> };
    let footer = html! { <p>{ "Some footer text" }</p> };

    let links = ChildrenRenderer::new(vec![
        html_nested! {<LoginMainFooterLink href="https://patternfly.org" target="_blank">{"Footer Link #1"}</LoginMainFooterLink>},
        html_nested! {<LoginMainFooterLink href="https://patternfly.org" target="_blank">{"Footer Link #2"}</LoginMainFooterLink>},
    ]);

    let band = ChildrenRenderer::new(vec![
        html! {<a href="#">{"Some link"}</a>},
        html! {<>{"Some other"}<a href="#">{" link"}</a></>},
    ]);

    let title = html_nested! { <Title size={Size::XXLarge}>{ "Login to your account" }</Title> };
    let toaster = use_toaster();

    let username = use_state_eq(String::new);
    let password = use_state_eq(String::new);

    let onchangeusername = {
        let username = username.clone();
        Callback::from(move |value| {
            username.set(value);
        })
    };

    let onchangepassword = {
        let password = password.clone();
        Callback::from(move |value| {
            password.set(value);
        })
    };

    let onsubmit = {
        let toaster = toaster.clone();
        let username = username.clone();
        let password = password.clone();
        Callback::from(move |_| {
            if let Some(toaster) = &toaster {
                toaster.toast(format!(
                    "Login - Username: {}, Password: {}",
                    &*username, &*password
                ));
            }
        })
    };

    html! {
        <>
            <ToastViewer>
                <Background />
                <Login {header} {footer}>
                    <LoginMain>
                        <LoginMainHeader
                            {title}
                            description="Enter the credentials to your account right here."
                        />
                        <LoginMainBody>
                            <Form {onsubmit} method="dialog">
                                <FormGroup label="Username">
                                    <TextInput
                                        required=true
                                        name="username"
                                        onchange={onchangeusername}
                                        value={(*username).clone()}
                                    />
                                </FormGroup>
                                <FormGroup label="Password">
                                    <TextInput
                                        required=true
                                        name="password"
                                        r#type={TextInputType::Password}
                                        onchange={onchangepassword}
                                        value={(*password).clone()}
                                    />
                                </FormGroup>
                                <ActionGroup>
                                    <Button
                                        label="Log In"
                                        r#type={ButtonType::Submit}
                                        variant={ButtonVariant::Primary}
                                    />
                                </ActionGroup>
                            </Form>
                        </LoginMainBody>
                        <LoginMainFooter {links} {band} />
                    </LoginMain>
                </Login>
            </ToastViewer>
        </>
    }
}
