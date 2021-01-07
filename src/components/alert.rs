use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use std::time::Duration;
use yew::prelude::*;

pub struct AlertExample {
    link: ComponentLink<Self>,
}

pub enum Msg {
    ShowToast(Toast),
}

impl Component for AlertExample {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowToast(toast) => {
                ToastDispatcher::new().toast(toast);
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let example1 = |title: &str, inline: bool| -> Html {
            return example! {title.to_string() =>
                <AlertGroup>
                    <Alert inline=inline title="Default alert">{"Some reason for the alert"}</Alert>
                    <Alert inline=inline title="Success alert" r#type=Type::Success>{"Some reason for the alert"}</Alert>
                    <Alert inline=inline title="Info alert" r#type=Type::Info>{"Some reason for the alert"}</Alert>
                    <Alert inline=inline title="Warning alert" r#type=Type::Warning>{"Some reason for the alert"}</Alert>
                    <Alert inline=inline title="Danger alert" r#type=Type::Danger>{"Some reason for the alert"}</Alert>
                </AlertGroup>
            };
        };

        let fix = self
            .link
            .callback(|_| {
                Msg::ShowToast("I am not sure another toast can fix this toast ;-)".into())
            })
            .into_action("Try fix!");

        let example2 = html! {
            <>
                <Button variant=Variant::Primary label="Toast" onclick=self.link.callback(|_|{
                    Msg::ShowToast("A toast".into())
                }) />
                <Button variant=Variant::Secondary label="Toast (5s)" onclick=self.link.callback(|_|{
                    Msg::ShowToast(Toast{
                        title: "A toast that will disappear in 5 seconds".into(),
                        timeout: Some(Duration::from_secs(5)),
                        ..Default::default()
                    })
                }) />
                <Button variant=Variant::Danger label="Toast (with children)" onclick=self.link.callback(move |_|{
                    Msg::ShowToast(Toast{
                        title: "A toast that will disappear in 5 seconds".into(),
                        timeout: Some(Duration::from_secs(5)),
                        r#type: Type::Danger,
                        body: html!{
                            <p>{"Some explanation of what went wrong..."}</p>
                        },
                        actions: vec![
                            fix.clone()
                        ],
                        ..Default::default()
                    })
                }) />
            </>
        };

        html! {
            <>
                <ExamplePage title="Alerts">
                    {example1("Alert", false)}
                    {example1("Alert (inline)", true)}
                </ExamplePage>
                <ExamplePage title="Toast">
                    {example2}
                </ExamplePage>
            </>
        }
    }
}
