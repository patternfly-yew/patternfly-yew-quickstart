use crate::{example, example::ExamplePage, layouts::LayoutItem};
use patternfly_yew::prelude::*;
use yew::prelude::*;

pub struct GalleryExample {}

impl Component for GalleryExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let example1 = example!("Gallery" => "gallery.1.example");
        let example2 = example!("Gallery (gutter)" => "gallery.2.example");
        html! {
            <>
                <ExamplePage title="Gallery Layout">{ example1 }{ example2 }</ExamplePage>
            </>
        }
    }
}
