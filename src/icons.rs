use crate::example::ExamplePage;
use patternfly_yew::prelude::*;
use strum::{EnumMessage, IntoEnumIterator};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IconDescriptor(Icon);

impl IconDescriptor {
    pub fn name(&self) -> &'static str {
        self.0.into()
    }
}

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
            Columns::Name => html!(<code>{ self.name() }</code>),
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
            <p>
                { "Sprinkle some icons into you application to make it look nice." }
                <Button
                    variant={ButtonVariant::Link}
                    label="PatternFly - Icons"
                    icon={Icon::ExternalLinkAlt}
                    align={Align::End}
                />
            </p>
        </div>
    );

    let entries = use_memo((), |()| {
        let mut icons = Icon::iter().map(IconDescriptor).collect::<Vec<_>>();
        icons.sort_by(|a, b| a.0.as_ref().cmp(b.0.as_ref()));
        icons
    });

    let header = html_nested! {
        <TableHeader<Columns>>
            <TableColumn<Columns> width={ColumnWidth::Percent(10)} label="" index={Columns::Icon} />
            <TableColumn<Columns>
                width={ColumnWidth::Percent(20)}
                label="Name"
                index={Columns::Name}
            />
            <TableColumn<Columns>
                width={ColumnWidth::Percent(70)}
                label="Description"
                index={Columns::Description}
            />
        </TableHeader<Columns>>
    };

    // search

    let filter = use_state_eq(String::new);

    let onclearfilter = use_callback(filter.clone(), |_, filter| filter.set(String::new()));
    let onsetfilter = use_callback(filter.clone(), |value: String, filter| {
        filter.set(value.trim().to_string())
    });

    // filter

    let entries = use_memo((entries.clone(), (*filter).clone()), |(entries, filter)| {
        let filter = filter.to_lowercase();

        entries
            .iter()
            .filter(|icon| icon.name().to_lowercase().contains(&filter))
            .cloned()
            .collect::<Vec<_>>()
    });

    // table data

    let (entries, _) = use_table_data(MemoizedTableModel::new(entries));

    // render

    html!(
        <ExamplePage title="Icons" {subtitle}>
            <div>
                <Toolbar>
                    <ToolbarContent>
                        <ToolbarItem r#type={ToolbarItemType::SearchFilter}>
                            <TextInputGroup>
                                <TextInputGroupMain
                                    placeholder="Filter"
                                    icon={Icon::Search}
                                    value={(*filter).clone()}
                                    onchange={onsetfilter}
                                />
                                if !filter.is_empty() {
                                    <TextInputGroupUtilities>
                                        <Button
                                            icon={Icon::Times}
                                            variant={ButtonVariant::Plain}
                                            onclick={onclearfilter}
                                        />
                                    </TextInputGroupUtilities>
                                }
                            </TextInputGroup>
                        </ToolbarItem>
                    </ToolbarContent>
                </Toolbar>
                <Table<Columns, UseTableData<Columns, MemoizedTableModel<IconDescriptor>>>
                    {header}
                    {entries}
                />
            </div>
        </ExamplePage>
    )
}
