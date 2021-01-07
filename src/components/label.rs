use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

use strum::IntoEnumIterator;

pub struct LabelExample {
    link: ComponentLink<Self>,
}

impl Component for LabelExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let example1 = |title, outline| {
            example! {title =>
                {
                    for Color::iter().map(|color|html!{
                    <Flex>
                        <FlexItem><Label outline=outline label=format!("{}", color) color=color/></FlexItem>
                        <FlexItem><Label outline=outline label=format!("{} with icon", color) color=color icon=Icon::InfoCircle/></FlexItem>
                        <FlexItem><Label outline=outline label=format!("{} closable", color) color=color onclose=self.link.callback(|_|{})/></FlexItem>
                        <FlexItem><Label outline=outline label=format!("{} closable with icon", color) color=color icon=Icon::InfoCircle onclose=self.link.callback(|_|{})/></FlexItem>
                        <FlexItem><Label outline=outline label=format!("{} clickable", color) color=color href="#"/></FlexItem>
                        <FlexItem><Label outline=outline label=format!("{} clickable, closable with icon", color) color=color icon=Icon::InfoCircle href="#" onclose=self.link.callback(|_|{})/></FlexItem>
                    </Flex>
                    })
                }
            }
        };

        html! {
            <>
                <ExamplePage title="Label">
                    {example1("Label", false)}
                    {example1("Label (outline)",true)}
                </ExamplePage>
            </>
        }
    }
}
