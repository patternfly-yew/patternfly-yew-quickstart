use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

pub struct ButtonExample {
    loading_primary: bool,
    loading_secondary: bool,
    loading_plain: bool,
}

pub enum ButtonMessage {
    LoadingPrimary,
    LoadingSecondary,
    LoadingPlain,
}

impl Component for ButtonExample {
    type Message = ButtonMessage;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { 
            loading_primary: true,
            loading_secondary: true,
            loading_plain: true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let variant = |title: &str, disabled: bool| -> Html {
            example! {title.to_string() =>
                <Button {disabled} variant={Variant::None}>{ "Default / None" }</Button>{" "}
                <Button {disabled} variant={Variant::Primary}>{ "Primary" }</Button>{" "}
                <Button {disabled} variant={Variant::Secondary}>{ "Secondary" }</Button>{" "}
                <Button {disabled} variant={Variant::Tertiary}>{ "Tertiary" }</Button>{" "}
                <Button {disabled} variant={Variant::Warning}>{ "Warning" }</Button>{" "}
                <Button {disabled} variant={Variant::Danger}>{ "Danger" }</Button>{" "}
                <Button {disabled} variant={Variant::DangerSecondary}>{ "DangerSecondary" }</Button>{" "}
                <Button {disabled} variant={Variant::Link}>{ "Link" }</Button>{" "}
                <Button {disabled} variant={Variant::InlineLink}>{ "InlineLink" }</Button>{" "}
                <Button {disabled} variant={Variant::Control}>{ "Control" }</Button>{" "}
                <Button {disabled} variant={Variant::Plain}>{ "Plain" }</Button>
            } 
        };

        let spin = example! { String::from("Progress Indicator") =>
            <Button
                loading={self.loading_primary}
                variant={Variant::Primary}
                onclick={ctx.link().callback(|_| ButtonMessage::LoadingPrimary)}
            >
                if self.loading_primary {
                    { "Pause loading logs" }
                } else {
                    { "Resume loading logs" }
                }
            </Button>{" "}
            <Button
                loading={self.loading_secondary}
                variant={Variant::Secondary}
                onclick={ctx.link().callback(|_| ButtonMessage::LoadingSecondary)}
            >
                if self.loading_primary {
                    { "Click to stop loading" }
                } else {
                    { "Click to start loading" }
                }
            </Button>{" "}
            /*
            <Button
                loading={self.loading_plain}
                variant={Variant::Plain}
                onclick={ctx.link().callback(|_| ButtonMessage::LoadingPlain)}
                icon={Icon::Bell} />
                */
        };

        /*
        let small = example! {String::from("Small Buttons") =>
            <Button  variant={Variant::Primary}>{ "Primary" }</Button>{" "}
            <Button  variant={Variant::Secondary}>{ "Secondary" }</Button>{" "}
            <Button  variant={Variant::Tertiary}>{ "Tertiary" }</Button>{" "}
            <Button  variant={Variant::Warning}>{ "Warning" }</Button>{" "}
            <Button  variant={Variant::Danger}>{ "Danger" }</Button>{" "}
        }
        */

        html! {
            <>
                <ExamplePage title="Buttons">
                    { variant("Variants example", false) }
                    { variant("Disabled buttons", true) }
                    { spin }
                </ExamplePage>
            </>
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ButtonMessage::LoadingPrimary => self.loading_primary = !self.loading_primary,
            ButtonMessage::LoadingSecondary => self.loading_secondary = !self.loading_secondary,
            ButtonMessage::LoadingPlain => self.loading_plain = !self.loading_plain,
        }

        true
    }
}
