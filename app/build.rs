fn main() {
    if cfg!(target_os = "windows") {
        let logo_svg_src = include_bytes!("assets/measurrred-logo.svg");
        let svg_tree = usvg::Tree::from_data(logo_svg_src, &usvg::Options::default())
            .expect("Should parse assets/measurrred-logo.svg as SVG tree");
        let size = svg_tree.size();
        let mut logo_pixmap = tiny_skia::Pixmap::new(size.width() as u32, size.height() as u32)
            .expect("The viewbox width and height should be greater than 0");
        resvg::render(
            &svg_tree,
            tiny_skia::Transform::default(),
            &mut logo_pixmap.as_mut(),
        );
        let logo_png = logo_pixmap.encode_png().expect("Should encode SVG as PNG");
        let logo_img = image::load_from_memory(&logo_png).expect("Should parse PNG buffer");
        logo_img
            .save_with_format(
                "assets/windows/icon/measurrred-logo.ico",
                image::ImageFormat::Ico,
            )
            .expect("Should save logo image as ICO format");

        embed_resource::compile(
            "assets/windows/icon/measurrred-icon.rc",
            embed_resource::NONE,
        );
        embed_resource::compile(
            "assets/windows/manifest/measurrred-manifest.rc",
            embed_resource::NONE,
        );
    }
}
