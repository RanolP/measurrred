fn main() {
    if cfg!(target_os = "windows") {
        let logo_svg_src = include_bytes!("assets/measurrred-logo.svg");
        let svg_tree = usvg::Tree::from_data(logo_svg_src, &usvg::Options::default().to_ref())
            .expect("Should parse assets/measurrred-logo.svg as SVG tree");
        let viewbox = svg_tree.svg_node().view_box;
        let mut logo_pixmap =
            tiny_skia::Pixmap::new(viewbox.rect.width() as u32, viewbox.rect.height() as u32)
                .expect("The viewbox width and height should be greater than 0");
        resvg::render(
            &svg_tree,
            usvg::FitTo::Original,
            tiny_skia::Transform::default(),
            logo_pixmap.as_mut(),
        );
        let logo_png = logo_pixmap.encode_png().expect("Should encode SVG as PNG");
        let logo_img = image::load_from_memory(&logo_png).expect("Should parse PNG buffer");
        logo_img
            .save_with_format(
                "assets/windows/icon/measurrred-logo.ico",
                image::ImageFormat::Ico,
            )
            .expect("Should save logo image as ICO format");

        embed_resource::compile("assets/windows/icon/measurrred-icon.rc");
        embed_resource::compile("assets/windows/manifest/measurrred-manifest.rc");
    }
}
