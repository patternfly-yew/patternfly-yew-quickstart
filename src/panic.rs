use crate::example::ExamplePage;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(Panic)]
pub fn panic() -> Html {
    let onclick = Callback::from(|_| panic!("Trying out panic!"));

    let actions = yew::props!(CardHeaderActionsObject {
        actions: html! {
            <Button {onclick} variant={ButtonVariant::Danger}> {"Panic!"} </Button>
        },
    });

    html!(
        <ExamplePage title="Panic Handler">
            <Grid gutter=true>
                <GridItem cols={[6]}>
                    <Card>
                        <CardTitle>
                            <Title>{ "Explanation" }</Title>
                        </CardTitle>
                        <CardBody>
                            <Content>
                                { Html::from_html_unchecked(r#"
<p>
The quickstart application sets a panic handler for Yew when starting. This at least allows one to
present the severe error to the user, maybe offering additional actions, like reporting the
issue.
</p>

<p>
This works by replacing the &lt;body&gt; element of the page with a templated paged, containing some
error information. If you've added PatternFly through the &lt;head&gt; element, you still have
all the styling available.
</p>

<p>
But keep in mind, at this point, the application crashed, and using Rust code is no longer an
option. You can still leverage the browsers' JavaScript engine and trigger some alternative
actions.
</p>

"#.into()) }
                            </Content>
                        </CardBody>
                    </Card>
                </GridItem>
                <GridItem cols={[6]}>
                    <Card>
                        <CardHeader {actions} />
                        <CardTitle>
                            <Title>{ "Example" }</Title>
                        </CardTitle>
                        <CardBody>
                            <Content>
                                { "By clicking on the button, you can cause a panic and try out the feature." }
                            </Content>
                        </CardBody>
                    </Card>
                </GridItem>
            </Grid>
        </ExamplePage>
    )
}
