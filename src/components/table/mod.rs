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

impl TableEntryRenderer for ExampleEntry {
    fn render_cell(&self, ctx: &CellContext) -> Cell {
        match ctx.column {
            0 => html!({ &self.foo }),
            1 => html!({ &self.foo.len() }),
            // this is a column which can't be seen
            3 => html! (
                <a href="#">{"Link"}</a>
            ),
            _ => html!(),
        }
        .into()
    }

    fn render_details(&self) -> Vec<Span> {
        vec![Span::max(html! (
            <>
                { "So many details for " }{ &self.foo }
            </>
        ))]
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
        let example1 = example!("Table" => "table.1.example");
        let example2 = example!("Compact Table" => "table.2.example");
        let example3 = example!("Compact, No Border Table" => "table.3.example");
        let example4 = example!("Compact, Expandable Table, Shared Model" => "table.4.example");
        let example5 = example!("Compact, Expandable Table, Shared Model" => "table.5.example");
        let example6 = example!("Table (centered)" => "table.6.example");
        let example7 = example!("Table (grid)" => "table.7.example");
        let example8 = example!("Table (widths)" => "table.8.example");

        html! {
            <>
                <ExamplePage title="Table">
                    {example1}
                    {example2}
                    {example3}
                    {example4}
                    {example5}
                    {example6}
                    {example7}
                    {example8}
                </ExamplePage>
            </>
        }
    }
}
