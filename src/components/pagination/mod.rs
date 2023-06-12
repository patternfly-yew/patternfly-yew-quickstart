use crate::{example, example::ExamplePage};
use patternfly_yew::{
    components::table::{
        use_table_data, Cell, CellContext, MemoizedTableModel, Table, TableColumn,
        TableEntryRenderer, TableHeader, UseTableData,
    },
    components::toolbar::{Toolbar, ToolbarContent},
    prelude::*,
};
use yew::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Columns {
    ItemNumber,
    Decimal,
    Hex,
}

struct ExampleEntry(usize);

impl TableEntryRenderer<Columns> for ExampleEntry {
    fn render_cell(&self, context: &CellContext<'_, Columns>) -> Cell {
        match context.column {
            Columns::ItemNumber => html!((self.0 + 1).to_string()),
            Columns::Decimal => html!(self.0.to_string()),
            Columns::Hex => html!(format!("{:x}", self.0)),
        }
        .into()
    }
}

#[function_component(PaginationExample)]
pub fn pagination_example() -> Html {
    let example1 = example!("Basic Example" => "pagination.1.example");

    html! {
        <ExamplePage title="Pagination">
            {example1}
        </ExamplePage>
    }
}
