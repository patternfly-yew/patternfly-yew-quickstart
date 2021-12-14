use crate::{example, example::ExamplePage};

use patternfly_yew::*;
use yew::prelude::*;

use chrono::Utc;

pub struct TableExample {
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

    fn create(_: &Context<Self>) -> Self {
        let model4 = vec![
            ExampleEntry {
                foo: "Simple foo".into(),
            },
            ExampleEntry {
                foo: "More foo".into(),
            },
        ];

        Self {
            model4: model4.into(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let entries = vec![
            ExampleEntry { foo: "bar".into() },
            ExampleEntry {
                foo: "Much, much longer foo".into(),
            },
        ];

        let model: SharedTableModel<_> = entries.into();

        let header = html_nested! {
            <TableHeader>
                <TableColumn label="foo"/>
                <TableColumn label="bar"/>
                <TableColumn label="baz"/>
            </TableHeader>
        };
        let example1 = example! {"Table" =>
            <Table<SharedTableModel<ExampleEntry>>
                caption="Table caption"
                header={header.clone()}
                entries={model.clone()}
                >
            </Table<SharedTableModel<ExampleEntry>>>
        };

        let example2 = example! {"Compact Table" =>
            <Table<SharedTableModel<ExampleEntry>>
                mode={TableMode::Compact}
                header={header.clone()}
                entries={model.clone()}
                >
            </Table<SharedTableModel<ExampleEntry>>>
        };

        let example3 = example! {"Compact, No Border Table" =>
            <Table<SharedTableModel<ExampleEntry>>
                mode={TableMode::CompactNoBorders}
                header={header.clone()}
                entries={model.clone()}
                >
            </Table<SharedTableModel<ExampleEntry>>>
        };

        let example4 = example! {"Compact, Expandable Table, Shared Model" =>
            <>
            <Table<SharedTableModel<ExampleEntry>>
                mode={TableMode::CompactExpandable}
                header={header.clone()}
                entries={self.model4.clone()}
                >
            </Table<SharedTableModel<ExampleEntry>>>

            <Button label="Prepend entry" icon={Icon::PlusCircleIcon} align={Align::Start} variant={Variant::Link} onclick={ctx.link().callback(|_| Msg::PrependToExample4)}/>
            <Button label="Append entry" icon={Icon::PlusCircleIcon} align={Align::Start} variant={Variant::Link} onclick={ctx.link().callback(|_| Msg::AppendToExample4)}/>
            <Button label="Pop entry" icon={Icon::PlusCircleIcon} align={Align::Start} variant={Variant::Link} onclick={ctx.link().callback(|_| Msg::PopFromExample4)}/>

            </>
        };

        let example5 = example! {"Compact, Expandable Table, Shared Model" =>
            <>
            <Table<SharedTableModel<ExampleEntry>>
                mode={TableMode::CompactExpandable}
                header={header.clone()}
                entries={self.model4.clone()}
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
