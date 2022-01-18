use serde::Deserialize;
use usvg::{Group, Node, NodeKind};

use self::data_text::DataText;
pub use self::hbox::HBox;
pub use self::render::{ComponentRender, RenderContext};
pub use self::setup::ComponentSetup;
pub use self::setup::SetupContext;
pub use self::text::Text;
use self::vbox::VBox;

mod data_text;
mod hbox;
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
    #[serde(rename = "margin")]
    Margin { size: f64 },
    #[serde(rename = "set-position")]
    SetPosition { to: f64 },
    #[serde(other)]
    Ignore,
}

impl ComponentSetup for Component {
    fn setup(&mut self, context: &mut SetupContext) -> eyre::Result<()> {
        match self {
            Component::Text(_) => Ok(()),
            Component::HBox(hbox) => hbox.setup(context),
            Component::VBox(vbox) => vbox.setup(context),
            Component::DataText(data_text) => data_text.setup(context),
            Component::SetPosition { .. } | Component::Margin { .. } | Component::Ignore => Ok(()),
        }
    }
}

impl ComponentRender for Component {
    fn render(&self, context: RenderContext) -> eyre::Result<Node> {
        match self {
            Component::Text(text) => text.render(context),
            Component::HBox(hbox) => hbox.render(context),
            Component::VBox(vbox) => vbox.render(context),
            Component::DataText(data_text) => data_text.render(context),
            Component::SetPosition { .. } | Component::Margin { .. } | Component::Ignore => {
                Ok(Node::new(NodeKind::Group(Group::default())))
            }
        }
    }
}
