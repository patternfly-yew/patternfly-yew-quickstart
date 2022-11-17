use patternfly_yew::*;
use yew::html::ChildrenRenderer;
use yew::prelude::*;

#[function_component(LoginPageExample)]
pub fn login_page_example() -> Html {
    let header = ChildrenRenderer::new(vec![html! {<> {"Header" }</>}]);
    let footer = ChildrenRenderer::new(vec![html! {<p>{"Some footer text"}</p>}]);

    let links = ChildrenRenderer::new(vec![
        html_nested! {<LoginMainFooterLink href="https://patternfly.org" target="_blank">{"Footer Link #1"}</LoginMainFooterLink>},
        html_nested! {<LoginMainFooterLink href="https://patternfly.org" target="_blank">{"Footer Link #2"}</LoginMainFooterLink>},
    ]);

    let band = ChildrenRenderer::new(vec![
        html! {<a href="#">{"Some link"}</a>},
        html! {<>{"Some other"}<a href="#">{" link"}</a></>},
    ]);

    let title = html_nested! {<Title size={Size::XXLarge}>{"Login to your account"}</Title>};

    html! {
        <>
            <Background/>
            <Login
                header={header}
                footer={footer}
                >
                <LoginMain>
                    <LoginMainHeader
                        title={title}
                        description="Enter the credentials to your account right here."
                        />
                    <LoginMainBody>
                        <Form>
                            <FormGroup label="Username">
                                <TextInput required=true name="username"/>
                            </FormGroup>
                            <FormGroup label="Password">
                                <TextInput required=true name="password" r#type="password"/>
                            </FormGroup>
                            <ActionGroup>
                                <Button label="Log In" r#type={ButtonType::Submit} variant={Variant::Primary}/>
                            </ActionGroup>
                        </Form>
                    </LoginMainBody>
                    <LoginMainFooter
                        links={links}
                        band={band}
                        >
                    </LoginMainFooter>
                </LoginMain>
            </Login>
        </>
    }
}
