use std::fmt;

use serde::Deserialize;
use usvg::{Node, NodeKind};

use self::data_graph::DataGraph;
use self::data_text::DataText;
use self::group::Group;
use self::hbox::HBox;
use self::import_font::ImportFont;
use self::text::Text;
use self::vbox::VBox;

pub use self::render::{ComponentRender, RenderContext};
pub use self::setup::{ComponentSetup, SetupContext};

mod data_graph;
mod data_text;
mod group;
mod hbox;
mod import_font;
mod text;
mod vbox;

mod render;
mod setup;

#[derive(Deserialize)]
pub enum Component {
    #[serde(rename = "text")]
    Text(Text),
    #[serde(rename = "hbox")]
    HBox(HBox),
    #[serde(rename = "vbox")]
    VBox(VBox),
    #[serde(rename = "data-text")]
    DataText(DataText),
    #[serde(rename = "data-graph")]
    DataGraph(DataGraph),
    #[serde(rename = "group")]
    Group(Group),

    #[serde(rename = "import-font")]
    ImportFont(ImportFont),

    #[serde(rename = "margin")]
    Margin { size: f64 },
    #[serde(rename = "set-position")]
    SetPosition { to: f64 },
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
            Self::DataText(_) => write!(f, "<data-text>"),
            Self::DataGraph(_) => write!(f, "<data-graph>"),
            Self::Group(_) => write!(f, "<data-graph>"),
            Self::ImportFont(_) => write!(f, "<import-font>"),
            Self::Margin { size } => write!(f, "<margin size={}>", size),
            Self::SetPosition { to } => write!(f, "<set-position to={}>", to),
            Self::Overlap { child } => write!(f, "<overlap>{:?}</overlap>", child),
            Self::Ignore => write!(f, "#Ignored"),
        }
    }
}

impl ComponentSetup for Component {
    fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()> {
        match self {
            Component::Text(_) => Ok(()),
            Component::HBox(hbox) => hbox.setup(context),
            Component::VBox(vbox) => vbox.setup(context),
            Component::DataText(data_text) => data_text.setup(context),
            Component::DataGraph(data_graph) => data_graph.setup(context),
            Component::Group(group) => group.setup(context),
            Component::ImportFont(import_font) => import_font.setup(context),
            Component::Overlap { child } => child.setup(context),
            Component::SetPosition { .. } | Component::Margin { .. } | Component::Ignore => Ok(()),
        }
    }
}

impl ComponentRender for Component {
    fn render(&mut self, context: &RenderContext) -> eyre::Result<Node> {
        match self {
            Component::Text(text) => text.render(context),
            Component::HBox(hbox) => hbox.render(context),
            Component::VBox(vbox) => vbox.render(context),
            Component::DataText(data_text) => data_text.render(context),
            Component::DataGraph(data_graph) => data_graph.render(context),
            Component::Overlap { child } => child.render(context),
            Component::Group(group) => group.render(context),

            Component::ImportFont(_)
            | Component::SetPosition { .. }
            | Component::Margin { .. }
            | Component::Ignore => Ok(Node::new(NodeKind::Group(usvg::Group::default()))),
        }
    }
}
