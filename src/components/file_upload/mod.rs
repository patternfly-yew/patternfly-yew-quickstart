use crate::{example, example::ExamplePage};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(FileUploadExample)]
pub fn view() -> Html {
    let example1 = example! {"Basic" => "file_upload.1.example" };

    html! (<ExamplePage title="File Upload">{ example1 }</ExamplePage>)
}
