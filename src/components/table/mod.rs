use crate::{example, example::ExamplePage};
use chrono::Utc;
use patternfly_yew::{
    components::table::{
        Cell, CellContext, ColumnWidth, MemoizedTableModel, Span, Table, TableColumn,
        TableEntryRenderer, TableHeader, UseStateTableModel, UseTableData, use_table_data,
    },
    prelude::*,
};
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExampleEntry {
    pub foo: String,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Columns {
    First,
    Second,
    Third,
}

impl TableEntryRenderer<Columns> for ExampleEntry {
    fn render_cell(&self, ctx: CellContext<Columns>) -> Cell {
        match ctx.column {
            Columns::First => html!({ &self.foo }),
            Columns::Second => html!({ &self.foo.len() }),
            // this is a column which can't be seen
            Columns::Third => html! (<a href="#">{ "Link" }</a>),
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

    fn render_column_details(&self, column: &Columns) -> Vec<Span> {
        vec![Span::max(match column {
            Columns::First => html!({ "First details" }),
            Columns::Second => html!({ "Second details" }),
            Columns::Third => html!({ "Third details" }),
        })]
    }
}

#[function_component(TableExample)]
pub fn example() -> Html {
    let model4 = use_state_eq(|| {
        vec![
            ExampleEntry {
                foo: "Simple foo".into(),
            },
            ExampleEntry {
                foo: "More foo".into(),
            },
        ]
    });

    let onprepend = {
        let model4 = model4.clone();
        Callback::from(move |_| {
            let mut model = (*model4).clone();
            model.insert(
                0,
                ExampleEntry {
                    foo: format!("Extra entry: {}", Utc::now()),
                },
            );
            model4.set(model);
        })
    };
    let onappend = {
        let model4 = model4.clone();
        Callback::from(move |_| {
            let mut model = (*model4).clone();
            model.push(ExampleEntry {
                foo: format!("Extra entry: {}", Utc::now()),
            });
            model4.set(model);
        })
    };
    let onpopentry = {
        let model4 = model4.clone();
        Callback::from(move |_| {
            let mut model = (*model4).clone();
            model.pop();
            model4.set(model);
        })
    };

    let (model4, onexpand) = use_table_data(UseStateTableModel::new(model4));

    let example1 = example!("Table" => "table.1.example");
    let example2 = example!("Compact Table" => "table.2.example");
    let example3 = example!("Compact, No Border Table" => "table.3.example");
    let example4 = example!("Compact, Expandable Table, Shared Model" => "table.4.example");
    let example5 = example!("Compact, Expandable Table, Shared Model" => "table.5.example");
    let example6 = example!("Table (centered)" => "table.6.example");
    let example7 = example!("Table (grid)" => "table.7.example");
    let example8 = example!("Table (widths)" => "table.8.example");
    let example9 = example!("Table (expandable)" => "table.9.example");
    let example10 = example!("Table (expandable, columns)" => "table.10.example");
    let example11 = example!("Table (sortable)" => "table.11.example");
    let example12 = example!("Table (clickable rows)" => "table.12.example");

    html! {
        <>
            <ExamplePage title="Table">
                { example1 }
                { example2 }
                { example3 }
                { example4 }
                { example5 }
                { example6 }
                { example7 }
                { example8 }
                { example9 }
                { example10 }
                { example11 }
                { example12 }
            </ExamplePage>
        </>
    }
}
