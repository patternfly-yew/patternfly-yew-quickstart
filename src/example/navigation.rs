use super::Example;
use patternfly_yew::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavigationDemo {
    BackToTop,
    BackToTopAlwaysVisible,
    BackToTopCustomTitle,
    JumpLinks,
    JumpLinksWithLabel,
    JumpLinksHorizontal,
    JumpLinksActive,
    JumpLinksNested,
    SkipToContent,
    SkipToContentCustomText,
}

impl NavigationDemo {
    pub fn title(&self) -> &'static str {
        match self {
            Self::BackToTop => "Back to top: scroll to reveal",
            Self::BackToTopAlwaysVisible => "Back to top: always visible",
            Self::BackToTopCustomTitle => "Back to top: custom title",
            Self::JumpLinks => "Jump links: basic",
            Self::JumpLinksWithLabel => "Jump links: with label",
            Self::JumpLinksHorizontal => "Jump links: horizontal",
            Self::JumpLinksActive => "Jump links: active item",
            Self::JumpLinksNested => "Jump links: nested subsections",
            Self::SkipToContent => "Skip to content: basic",
            Self::SkipToContentCustomText => "Skip to content: custom text",
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct NavigationPageProperties {
    pub demo: NavigationDemo,
}

#[function_component(NavigationExample)]
pub fn navigation_example(props: &NavigationPageProperties) -> Html {
    let (content, source) = props.demo.example();
    let node_ref = use_node_ref();
    let body = use_state_eq(|| None::<web_sys::Element>);
    {
        let body = body.clone();
        use_effect_with(node_ref.clone(), move |node_ref| {
            if let Some(frame) = node_ref.cast::<web_sys::HtmlIFrameElement>() {
                match prepare_frame(&frame) {
                    Ok(element) => body.set(Some(element)),
                    Err(error) => log::error!("Cannot prepare navigation preview: {error:?}"),
                }
            }
        });
    }
    html! {
        <Example title={props.demo.title().to_owned()} code={source.to_owned()}>
            <iframe
                ref={node_ref}
                title={props.demo.title()}
                style="width: 100%; height: 24rem; border: 0;"
            />
            if let Some(body) = &*body {
                { yew::create_portal(html! { <div style="padding: 1rem;">{content}</div> }, body.clone()) }
            }
        </Example>
    }
}

fn prepare_frame(frame: &web_sys::HtmlIFrameElement) -> Result<web_sys::Element, JsValue> {
    let document = frame
        .content_document()
        .ok_or_else(|| JsValue::from_str("Missing preview document"))?;
    let head = document
        .head()
        .ok_or_else(|| JsValue::from_str("Missing preview head"))?;
    let parent = gloo_utils::document();
    let base = document.create_element("base")?;
    base.set_attribute("href", "about:blank")?;
    head.append_child(&base)?;
    let styles = parent.query_selector_all("link[rel=stylesheet], style")?;
    for index in 0..styles.length() {
        if let Some(style) = styles.item(index) {
            let copy = style.clone_node_with_deep(true)?;
            if let Some(link) = style.dyn_ref::<web_sys::HtmlLinkElement>() {
                copy.unchecked_ref::<web_sys::Element>()
                    .set_attribute("href", &link.href())?;
            }
            head.append_child(&copy)?;
        }
    }
    if let (Some(source), Some(target)) = (parent.document_element(), document.document_element()) {
        target.set_class_name(&source.class_name());
        target.set_attribute("lang", "en")?;
    }
    document
        .body()
        .map(Into::into)
        .ok_or_else(|| JsValue::from_str("Missing preview body"))
}

impl NavigationDemo {
    fn example(&self) -> (Html, &'static str) {
        match self {
            NavigationDemo::BackToTop => (
                include!("../components/back_to_top/back_to_top.2.example"),
                include_str!("../components/back_to_top/back_to_top.2.example"),
            ),
            NavigationDemo::BackToTopAlwaysVisible => (
                include!("../components/back_to_top/back_to_top.1.example"),
                include_str!("../components/back_to_top/back_to_top.1.example"),
            ),
            NavigationDemo::BackToTopCustomTitle => (
                include!("../components/back_to_top/back_to_top.3.example"),
                include_str!("../components/back_to_top/back_to_top.3.example"),
            ),
            NavigationDemo::JumpLinks => (
                include!("../components/jump_links/jump_links.1.example"),
                include_str!("../components/jump_links/jump_links.1.example"),
            ),
            NavigationDemo::JumpLinksWithLabel => (
                include!("../components/jump_links/jump_links.2.example"),
                include_str!("../components/jump_links/jump_links.2.example"),
            ),
            NavigationDemo::JumpLinksHorizontal => (
                include!("../components/jump_links/jump_links.3.example"),
                include_str!("../components/jump_links/jump_links.3.example"),
            ),
            NavigationDemo::JumpLinksActive => (
                include!("../components/jump_links/jump_links.4.example"),
                include_str!("../components/jump_links/jump_links.4.example"),
            ),
            NavigationDemo::JumpLinksNested => (
                include!("../components/jump_links/jump_links.5.example"),
                include_str!("../components/jump_links/jump_links.5.example"),
            ),
            NavigationDemo::SkipToContent => (
                include!("../components/skip_to_content/skip_to_content.1.example"),
                include_str!("../components/skip_to_content/skip_to_content.1.example"),
            ),
            NavigationDemo::SkipToContentCustomText => (
                include!("../components/skip_to_content/skip_to_content.2.example"),
                include_str!("../components/skip_to_content/skip_to_content.2.example"),
            ),
        }
    }
}
