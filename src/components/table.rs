use crate::example;

use patternfly_yew::*;
use yew::prelude::*;

pub struct TableExample {}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExampleEntry {
    pub foo: String,
}

impl TableRenderer for ExampleEntry {
    fn render(&self, column: ColumnIndex) -> Html {
        match column.index {
            0 => html! {{&self.foo}},
            1 => html! {{&self.foo.len()}},
            _ => html! {},
        }
    }

    fn render_details(&self) -> Vec<Span> {
        vec![Span::max(html! {
            <>
                { "So many details for " }{ &self.foo }
            </>
        })]
    }
}

impl Component for TableExample {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let entries = vec![
            ExampleEntry { foo: "bar".into() },
            ExampleEntry {
                foo: "Much, much longer foo".into(),
            },
        ];

        let example1 = example! {"Table" |
            <Table<ExampleEntry>
                caption="Table caption"
                header={html_nested!{
                    <TableHeader>
                        <TableColumn label="foo"/>
                        <TableColumn label="bar"/>
                        <TableColumn label="baz"/>
                    </TableHeader>
                }}
                entries=entries.clone()
                >
            </Table<ExampleEntry>>
        };

        let example2 = example! {"Compact Table" |
            <Table<ExampleEntry>
                mode=TableMode::Compact
                header={html_nested!{
                    <TableHeader>
                        <TableColumn label="foo"/>
                        <TableColumn label="bar"/>
                        <TableColumn label="baz"/>
                    </TableHeader>
                }}
                entries=entries.clone()
                >
            </Table<ExampleEntry>>
        };

        let example3 = example! {"Compact, No Border Table" |
            <Table<ExampleEntry>
                mode=TableMode::CompactNoBorders
                header={html_nested!{
                    <TableHeader>
                        <TableColumn label="foo"/>
                        <TableColumn label="bar"/>
                        <TableColumn label="baz"/>
                    </TableHeader>
                }}
                entries=entries.clone()
                >
            </Table<ExampleEntry>>
        };

        let example4 = example! {"Compact, Expandable Table" |
            <Table<ExampleEntry>
                mode=TableMode::CompactExpandable
                header={html_nested!{
                    <TableHeader>
                        <TableColumn label="foo"/>
                        <TableColumn label="bar"/>
                        <TableColumn label="baz"/>
                    </TableHeader>
                }}
                entries=entries.clone()
                >
            </Table<ExampleEntry>>
        };

        html! {
            <>
                <PageSection variant=PageSectionVariant::Light limit_width=true>
                    <Content>
                        <h1>{"Table"}</h1>
                    </Content>
                </PageSection>
                <PageSection>
                    <Content>
                        {example1}
                        {example2}
                        {example3}
                        {example4}
                    </Content>
                </PageSection>
            </>
        }
    }
}
