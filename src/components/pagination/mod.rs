use crate::{example, example::ExamplePage};
use patternfly_yew::{
    components::table::{
        Cell, CellContext, MemoizedTableModel, Table, TableColumn, TableEntryRenderer, TableHeader,
        UseTableData, use_table_data,
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

#[derive(Clone)]
struct ExampleEntry(usize);

impl TableEntryRenderer<Columns> for ExampleEntry {
    fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
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
    let example2 = example!("Hook Example" => "pagination.2.example");

    html! { <ExamplePage title="Pagination">{ example1 }{ example2 }</ExamplePage> }
}
