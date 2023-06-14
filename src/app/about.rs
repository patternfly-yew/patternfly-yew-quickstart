use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html!(
        <Bullseye>
            <AboutModal
                background_image_src="/assets/images/pfbg-icon.svg"
                brand_image_src="/assets/images/PF-Masthead-Logo.svg"
                brand_image_alt="PatternFly logo"
                product_name="PatternFly Yew Quickstart"
                trademark="Copyright © 2020, 2023 PatternFly for Yew contributors"
            >
                <Content>
                    <p>{ env!("CARGO_PKG_DESCRIPTION") }</p>
                    <dl style="width: 100%">
                        <dt>{ "Version" }</dt>
                        <dd>{ env!("CARGO_PKG_VERSION") }</dd>
                        <dt>{ "License" }</dt>
                        <dd>{ env!("CARGO_PKG_LICENSE") }</dd>
                        if let Some(value) = option_env!("BUILD_COMMIT") {
                            <dt>{ "Build commit" }</dt>
                            <dd>{ value }</dd>
                        }
                        if let Some(value) = option_env!("BUILD_TIMESTAMP") {
                            <dt>{ "Build timestamp" }</dt>
                            <dd>{ value }</dd>
                        }
                    </dl>
                </Content>
            </AboutModal>
        </Bullseye>
    )
}
