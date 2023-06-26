use crate::example::ExamplePage;
use patternfly_yew::prelude::*;
use strum::{EnumMessage, IntoEnumIterator};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IconDescriptor(Icon);

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Columns {
    Icon,
    Name,
    Description,
}

impl TableEntryRenderer<Columns> for IconDescriptor {
    fn render_cell(&self, context: CellContext<Columns>) -> Cell {
        match context.column {
            Columns::Icon => self.0.as_html(),
            Columns::Name => html!(<code>{self.0.as_ref()}</code>),
            Columns::Description => self
                .0
                .get_documentation()
                .map(Html::from)
                .unwrap_or_default(),
        }
        .into()
    }
}

#[function_component(Icons)]
pub fn icons() -> Html {
    let subtitle = html!(
        <div>
            <p>{"Sprinkle some icons into you application to make it look nice."}
            <Button variant={ButtonVariant::Link} label="PatternFly - Icons" icon={Icon::ExternalLinkAlt} align={Align::End} />
            </p>
        </div>
    );

    let entries = use_memo(
        |()| {
            let mut icons = Icon::iter().map(IconDescriptor).collect::<Vec<_>>();
            icons.sort_by(|a, b| a.0.as_ref().cmp(b.0.as_ref()));
            icons
        },
        (),
    );

    let (entries, _) = use_table_data(MemoizedTableModel::new(entries));

    let header = html_nested! {
        <TableHeader<Columns>>
            <TableColumn<Columns> index={Columns::Icon}/>
            <TableColumn<Columns> label="Name" index={Columns::Name}/>
            <TableColumn<Columns> label="Description" index={Columns::Description}/>
        </TableHeader<Columns>>
    };

    html!(
        <ExamplePage title="Icons" {subtitle}>
            <Table<Columns, UseTableData<Columns, MemoizedTableModel<IconDescriptor>>>
                {header}
                {entries}
            />
        // <Table<SharedTableModel<IconDescriptor>> header={(*header).clone()} entries={(*entries).clone()}/>
        </ExamplePage>
    )
}
