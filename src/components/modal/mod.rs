use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(ModalExample)]
pub fn modal_example() -> Html {
    let example1 = example!("Basic Medium Modal" => "modal.1.example");
    let example2 = example!("Modal with a form" => "modal.2.example");

    let subtitle = Html::from_html_unchecked(
        r#"
<p>
    The actual modal content. See <code>Backdrop</code> for showing a modal.
</p>
"#
        .into(),
    );

    html! {
        <>
            <ExamplePage title="Modal" {subtitle}>{ example1 }{ example2 }</ExamplePage>
        </>
    }
}
