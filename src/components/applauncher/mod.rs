use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

#[function_component(AppLauncherExample)]
pub fn app_launcher_example() -> Html {
    let toaster = use_toaster();

    let toast = move |message: &str| {
        if let Some(toaster) = &toaster {
            toaster.toast(Toast {
                title: message.into(),
                r#type: AlertType::Success,
                ..Default::default()
            });
        } else {
            log::info!("Missing toaster");
        }
    };

    let example1 = example! ("AppLauncher" => "applauncher.1.example");

    html! {
        <>
            <ExamplePage title="AppLauncher">
                <Alert inline=true title="AppLauncher Depreciated" r#type={AlertType::Warning}>
                    {"The AppLauncher component has been depreciated by PatternFly."}
                    <br />
                    {"See "}
                    <a href="https://patternfly-react-v5.surge.sh/components/menus/application-launcher">
                        {"ApplicationLauncher component page"}
                    </a>
                    {" in the PatternFly documents for more information."}
                </Alert>
                {example1}
            </ExamplePage>
        </>
    }
}
