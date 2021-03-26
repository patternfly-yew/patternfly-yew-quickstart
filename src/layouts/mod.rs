mod bullseye;
mod flex;
mod gallery;
mod grid;
mod split;
mod stack;

pub use bullseye::*;
pub use flex::*;
pub use gallery::*;
pub use grid::*;
pub use split::*;
pub use stack::*;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

/// A general purpose layout item for creating examples on layouts.
pub struct LayoutItem {
    props: Props,
}

impl Component for LayoutItem {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        return html! {
            <div style="border: .2rem dashed gray; padding: 1rem; height: 100%;">
                { for self.props.children.iter() }
            </div>
        };
    }
}
