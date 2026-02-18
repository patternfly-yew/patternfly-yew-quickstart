use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html!(
        <Bullseye>
            <AboutModal
                background_image_src="assets/images/pfbg-icon.svg"
                brand_image_src="assets/images/pf-logo.svg"
                brand_image_alt="PatternFly logo"
                product_name="PatternFly Yew Quickstart"
                trademark="Copyright © 2020, 2023 PatternFly for Yew contributors"
            >
                <p>{ env!("CARGO_PKG_DESCRIPTION") }</p>
                <br />
                <DescriptionList mode={[DescriptionListMode::Horizontal]}>
                    <DescriptionGroup term="Version">
                        { env!("CARGO_PKG_VERSION") }
                    </DescriptionGroup>
                    <DescriptionGroup term="Name">{ env!("CARGO_PKG_VERSION") }</DescriptionGroup>
                    <DescriptionGroup term="License">
                        { env!("CARGO_PKG_LICENSE") }
                    </DescriptionGroup>
                    if let Some(value) = option_env!("BUILD_COMMIT") {
                        <DescriptionGroup term="Build commit">{ value }</DescriptionGroup>
                    }
                    if let Some(value) = option_env!("BUILD_TIMESTAMP") {
                        <DescriptionGroup term="Build timestamp">{ value }</DescriptionGroup>
                    }
                </DescriptionList>
            </AboutModal>
        </Bullseye>
    )
}
