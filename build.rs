fn main() {
    if cfg!(target_os = "windows") {
        embed_resource::compile("meassurred-manifest.rc");
    }
}
