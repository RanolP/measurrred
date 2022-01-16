use usvg::{Node, Options, Tree};

pub enum Component {
    HBox(),
    VBox(),
    DataText(),
}

impl Component {
    pub fn render(&mut self) -> Node {
        let mut options = Options::default();
        options.fontdb.load_system_fonts();
        #[cfg(target_os = "windows")]
        {
            let local_appdata = std::env::var("LocalAppdata").unwrap();
            options.fontdb.load_fonts_dir(
                std::path::PathBuf::from(local_appdata).join("Microsoft/Windows/Fonts"),
            );
        }
        Tree::from_str(
            r#"
                <svg version="1.1" width="100" height="32" xmlns="http://www.w3.org/2000/svg">
                    <text id="root" fill="red" font-size="24" font-family="Noto Sans CJK KR Bold">Hello</text>
                </svg>
            "#,
            &options.to_ref(),
        )
        .unwrap()
        .node_by_id("root")
        .unwrap()
    }
}
