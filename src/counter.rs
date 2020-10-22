use yew::prelude::*;
use patternfly_yew::*;

pub struct Counter {
    link: ComponentLink<Self>,
    value: i64,
}

pub enum Msg {
    AddOne,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
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
            </>
        }
    }
}