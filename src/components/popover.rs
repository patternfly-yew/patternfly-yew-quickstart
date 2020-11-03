use crate::{example, example::Example};

use patternfly_yew::*;
use yew::prelude::*;

pub struct PopoverExample {}

impl Component for PopoverExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let example1 = example! {"Popover Popup" =>
            <PopoverPopup orientation=Orientation::Left
                header=html!{<Title size=Size::Medium>{"Popover Title"}</Title>}
                footer=html!{"Some footer content."}
                >
                {"This is just an example popover."}
            </PopoverPopup>
        };

        let example2 = example! {"Popover" =>
            <Popover
                toggle_by_onclick=true
                header=html!{<Title size=Size::Medium>{"Popover Title"}</Title>}
                footer=html!{"Some footer content."}
                target=html!{
                    <span
                        style="border: 1px solid black;">
                        {"I have a popover. Click me."}
                    </span>
                }
                >
                {"This is just an example popover."}
            </Popover>
        };

        html! {
            <>
                <Example title="Tooltip">
                    {example1}
                    {example2}
                </Example>
            </>
        }
    }
}
