mod bullseye;
mod flex;
mod gallery;
mod grid;
mod level;
mod masonry;
mod split;
mod stack;

pub use bullseye::*;
pub use flex::*;
pub use gallery::*;
pub use grid::*;
pub use level::*;
pub use masonry::*;
pub use split::*;
pub use stack::*;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(LayoutItem)]
pub fn layout_item(props: &Props) -> Html {
    html! {
        <div style="border: .2rem dashed gray; padding: 1rem; height: 100%">
            { for props.children.iter() }
        </div>
    }
}
