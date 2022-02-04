use std::fmt;

use serde::Deserialize;
use usvg::{Node, NodeKind};

use crate::system::Length;

use self::actual::*;

pub use self::action::{ComponentAction, RenderContext, SetupContext, UpdateContext};

mod action;
mod actual;

#[derive(Deserialize)]
pub enum Component {
    #[serde(rename = "text")]
    Text(Text),
    #[serde(rename = "hbox")]
    HBox(HBox),
    #[serde(rename = "vbox")]
    VBox(VBox),
    #[serde(rename = "fetch-data")]
    FetchData(FetchData),
    #[serde(rename = "graph")]
    Graph(Graph),
    #[serde(rename = "group")]
    Group(Group),

    #[serde(rename = "import-font")]
    ImportFont(ImportFont),

    #[serde(rename = "margin")]
    Margin { size: Length },
    #[serde(rename = "set-position")]
    SetPosition { to: Length },
    #[serde(rename = "overlap")]
    Overlap {
        #[serde(rename = "$value")]
        child: Box<Component>,
    },

    #[serde(other)]
    Ignore,
}

impl fmt::Debug for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(_) => write!(f, "<text>"),
            Self::HBox(_) => write!(f, "<hbox>"),
            Self::VBox(_) => write!(f, "<vbox>"),
            Self::FetchData(_) => write!(f, "<fetch-data>"),
            Self::Graph(_) => write!(f, "<graph>"),
            Self::Group(_) => write!(f, "<group>"),
            Self::ImportFont(_) => write!(f, "<import-font>"),
            Self::Margin { size } => write!(f, "<margin size={}>", size),
            Self::SetPosition { to } => write!(f, "<set-position to={}>", to),
            Self::Overlap { child } => write!(f, "<overlap>{:?}</overlap>", child),
            Self::Ignore => write!(f, "#Ignored"),
        }
    }
}

impl ComponentAction for Component {
    fn setup<'a>(
        &'a mut self,
    ) -> eyre::Result<Box<dyn FnOnce(&mut SetupContext) -> eyre::Result<()> + Send + 'a>> {
        match self {
            Component::Text(text) => text.setup(),
            Component::HBox(hbox) => hbox.setup(),
            Component::VBox(vbox) => vbox.setup(),
            Component::FetchData(data_text) => data_text.setup(),
            Component::Graph(data_graph) => data_graph.setup(),
            Component::Group(group) => group.setup(),
            Component::ImportFont(import_font) => import_font.setup(),
            Component::Overlap { child } => child.setup(),
            Component::Margin { .. } | Component::SetPosition { .. } | Component::Ignore => {
                Ok(Box::new(|_| Ok(())))
            }
        }
    }

    fn update(&mut self, context: &mut UpdateContext) -> eyre::Result<()> {
        match self {
            Component::Text(text) => text.update(context),
            Component::HBox(hbox) => hbox.update(context),
            Component::VBox(vbox) => vbox.update(context),
            Component::FetchData(data_text) => data_text.update(context),
            Component::Graph(data_graph) => data_graph.update(context),
            Component::Group(group) => group.update(context),
            Component::ImportFont(import_font) => import_font.update(context),
            Component::Overlap { child } => child.update(context),
            Component::Margin { .. } | Component::SetPosition { .. } | Component::Ignore => Ok(()),
        }
    }

    fn render(&mut self, context: &RenderContext) -> eyre::Result<Node> {
        match self {
            Component::Text(text) => text.render(context),
            Component::HBox(hbox) => hbox.render(context),
            Component::VBox(vbox) => vbox.render(context),
            Component::FetchData(fetch_data) => fetch_data.render(context),
            Component::Graph(graph) => graph.render(context),
            Component::Group(group) => group.render(context),
            Component::Overlap { child } => child.render(context),

            Component::ImportFont(_)
            | Component::SetPosition { .. }
            | Component::Margin { .. }
            | Component::Ignore => Ok(Node::new(NodeKind::Group(usvg::Group::default()))),
        }
    }
}
