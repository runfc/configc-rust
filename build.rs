use std::path::Path;

fn main() {
    if !Path::new("./configc.c").exists() {
       panic!("Unable to find ./configc.c on the current directory, which is required for building");
    }

    cc::Build::new()
        .file("./configc.c/src/log.c")
	.file("./configc.c/src/lib.c")
	.file("./configc.c/src/file.c")
        .file("./configc.c/src/sysctl.c")
        .compile("configc");
}
