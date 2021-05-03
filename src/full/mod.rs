mod login_page;

pub use self::login_page::*;

use crate::example::*;
use patternfly_yew::*;
use yew::prelude::*;

pub struct FullPageExample {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    Open,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub url: String,
}

impl Component for FullPageExample {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Open => {
                self.open();
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <ExamplePage title="Login Page">
                    <Button variant=Variant::Primary icon=Icon::ExternalLinkAlt align=Align::Start label="Open" onclick=self.link.callback(|_| Msg::Open) />
                </ExamplePage>
            </>
        }
    }
}

impl FullPageExample {
    fn open(&self) {
        yew::utils::window()
            .open_with_url_and_target(&self.props.url, "patternfly-yew-example")
            .ok();
    }
}
