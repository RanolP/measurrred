use once_cell::sync::OnceCell;
use serde::Deserialize;
use usvg::{Node, Options, Tree};

#[derive(Deserialize)]
#[serde(rename = "text", rename_all = "kebab-case")]
pub struct Text {
    color: Option<String>,
    font_size: Option<String>,
    #[serde(rename = "$value")]
    content: String,
    #[serde(skip)]
    node: OnceCell<Node>,
}

impl Text {
    pub fn from_content(s: impl AsRef<str>) -> Self {
        let content = s.as_ref().to_string();
        Text {
            color: None,
            font_size: None,
            content,
            node: OnceCell::new(),
        }
    }
}

impl Text {
    pub fn render(&mut self, options: &Options) -> eyre::Result<Node> {
        let node = self.node.get_or_try_init(|| -> eyre::Result<_> {
            let svg = format!(
                r#"
                <svg version="1.1" width="100" height="32" xmlns="http://www.w3.org/2000/svg">
                    <text
                        id="root"
                        fill="{color}"
                        font-size="{font_size}"
                        font-family="Noto Sans CJK KR Bold"
                    >
                        {content}
                    </text>
                </svg>
                "#,
                content = self.content,
                color = self.color.as_ref().unwrap_or(&"white".to_string()),
                font_size = self.font_size.as_ref().unwrap_or(&"16px".to_string()),
            );
            let tree = Tree::from_str(&svg, &options.to_ref())?;
            let node = tree.node_by_id("root").unwrap();
            Ok(node)
        })?;
        Ok(node.clone())
    }
}
