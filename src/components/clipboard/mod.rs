use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(ClipboardExample)]
pub fn clipboard_example() -> Html {
    let example1 = example! ("Clipboard" => "clipboard.1.example");
    let example2 = example! ("Clipboard (readonly)" => "clipboard.2.example");
    let example3 = example! ("Clipboard (code, expandable)" => "clipboard.3.example");
    let example4 = example! ("Clipboard (readonly, expanded)" => "clipboard.4.example");

    html! {
        <>
            <ExamplePage title="Clipboard">
                { example1 }
                { example2 }
                { example3 }
                { example4 }
            </ExamplePage>
        </>
    }
}
