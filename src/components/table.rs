use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

use chrono::Utc;

pub struct TableExample {
    link: ComponentLink<Self>,
    model4: SharedTableModel<ExampleEntry>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExampleEntry {
    pub foo: String,
}

impl TableRenderer for ExampleEntry {
    fn render(&self, column: ColumnIndex) -> Html {
        match column.index {
            0 => html! {{&self.foo}},
            1 => html! {{&self.foo.len()}},
            3 => html! {
                <a href="#">{"Link"}</a>
            },
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

pub enum Msg {
    AppendToExample4,
    PrependToExample4,
    PopFromExample4,
}

impl Component for TableExample {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let model4 = vec![
            ExampleEntry {
                foo: "Simple foo".into(),
            },
            ExampleEntry {
                foo: "More foo".into(),
            },
        ];

        Self {
            link,
            model4: model4.into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AppendToExample4 => self.model4.push(ExampleEntry {
                foo: format!("Extra entry: {}", Utc::now()),
            }),
            Msg::PrependToExample4 => self.model4.insert(
                0,
                ExampleEntry {
                    foo: format!("Extra entry: {}", Utc::now()),
                },
            ),
            Msg::PopFromExample4 => {
                self.model4.pop();
            }
        }
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

        let model: SimpleTableModel<_> = entries.into();

        let example1 = example! {"Table" =>
            <Table<SimpleTableModel<ExampleEntry>>
                caption="Table caption"
                header={html_nested!{
                    <TableHeader>
                        <TableColumn label="foo"/>
                        <TableColumn label="bar"/>
                        <TableColumn label="baz"/>
                    </TableHeader>
                }}
                entries=model.clone()
                >
            </Table<SimpleTableModel<ExampleEntry>>>
        };

        let example2 = example! {"Compact Table" =>
            <Table<SimpleTableModel<ExampleEntry>>
                mode=TableMode::Compact
                header={html_nested!{
                    <TableHeader>
                        <TableColumn label="foo"/>
                        <TableColumn label="bar"/>
                        <TableColumn label="baz"/>
                    </TableHeader>
                }}
                entries=model.clone()
                >
            </Table<SimpleTableModel<ExampleEntry>>>
        };

        let example3 = example! {"Compact, No Border Table" =>
            <Table<SimpleTableModel<ExampleEntry>>
                mode=TableMode::CompactNoBorders
                header={html_nested!{
                    <TableHeader>
                        <TableColumn label="foo"/>
                        <TableColumn label="bar"/>
                        <TableColumn label="baz"/>
                    </TableHeader>
                }}
                entries=model.clone()
                >
            </Table<SimpleTableModel<ExampleEntry>>>
        };

        let example4 = example! {"Compact, Expandable Table, Shared Model" =>
            <>
            <Table<SharedTableModel<ExampleEntry>>
                mode=TableMode::CompactExpandable
                header={html_nested!{
                    <TableHeader>
                        <TableColumn label="foo"/>
                        <TableColumn label="bar"/>
                        <TableColumn label="baz"/>
                    </TableHeader>
                }}
                entries=self.model4.clone()
                >
            </Table<SharedTableModel<ExampleEntry>>>

            <Button label="Prepend entry" icon=Icon::PlusCircleIcon align=Align::Start variant=Variant::Link onclick=self.link.callback(|_| Msg::PrependToExample4)/>
            <Button label="Append entry" icon=Icon::PlusCircleIcon align=Align::Start variant=Variant::Link onclick=self.link.callback(|_| Msg::AppendToExample4)/>
            <Button label="Pop entry" icon=Icon::PlusCircleIcon align=Align::Start variant=Variant::Link onclick=self.link.callback(|_| Msg::PopFromExample4)/>

            </>
        };

        let example5 = example! {"Compact, Expandable Table, Shared Model" =>
            <>
            <Table<SharedTableModel<ExampleEntry>>
                mode=TableMode::CompactExpandable
                header={html_nested!{
                    <TableHeader>
                        <TableColumn label="foo"/>
                        <TableColumn label="bar"/>
                        <TableColumn label="baz"/>
                        <TableColumn/>
                    </TableHeader>
                }}
                entries=self.model4.clone()
                >
            </Table<SharedTableModel<ExampleEntry>>>

            </>
        };

        html! {
            <>
                <ExamplePage title="Table">
                    {example1}
                    {example2}
                    {example3}
                    {example4}
                    {example5}
                </ExamplePage>
            </>
        }
    }
}
