use serde::Deserialize;
use usvg::{Node, Options, Tree};

pub use self::text::Text;

mod data_text;
mod hbox;
mod text;
mod vbox;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Component {
    Text(Text),
}

pub trait ComponentRender {
    fn render(&mut self, parent: Option<&Component>, usvg_options: &Options) -> eyre::Result<Node>;
}

impl Component {
    pub fn render(&mut self, options: &Options) -> eyre::Result<Node> {
        match self {
            Component::Text(plain_text) => return plain_text.render(options),
        }
    }
}
