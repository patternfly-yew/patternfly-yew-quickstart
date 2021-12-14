mod login_page;

pub use self::login_page::*;

use crate::example::*;
use patternfly_yew::*;
use yew::prelude::*;

pub struct FullPageExample {}

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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Open => {
                self.open(ctx);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <ExamplePage title="Login Page">
                    <Button variant={Variant::Primary} icon={Icon::ExternalLinkAlt} align={Align::Start} label="Open" onclick={ctx.link().callback(|_| Msg::Open)} />
                </ExamplePage>
            </>
        }
    }
}

impl FullPageExample {
    fn open(&self, ctx: &Context<Self>) {
        gloo_utils::window()
            .open_with_url_and_target(&ctx.props().url, "patternfly-yew-example")
            .ok();
    }
}
