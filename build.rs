fn main() {
    #[cfg(target_os = "windows")]
    {
        extern crate embed_resource;
        embed_resource::compile("tools/build-windows/wolkenwelten.rc");
    }
}
