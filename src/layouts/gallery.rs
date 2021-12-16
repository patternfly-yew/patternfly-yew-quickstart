use crate::{example::ExamplePage, example2, layouts::LayoutItem};
use patternfly_yew::*;
use yew::prelude::*;

pub struct GalleryExample {}

impl Component for GalleryExample {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let example1 = example2!("Gallery" => "gallery.1.example");
        let example2 = example2!("Gallery (gutter)" => "gallery.2.example");
        html! {
            <>
                <ExamplePage title="Gallery Layout">
                    { example1 }
                    { example2 }
                </ExamplePage>
            </>
        }
    }
}
