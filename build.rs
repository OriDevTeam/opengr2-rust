const LIB_NAME: &str = "opengr2";

fn main() {
    pkg_config::Config::new()
        .atleast_version("1.2");
        //.probe("z")
        //.unwrap();
    
    let src = [
        "thirdparty\\opengr2\\libopengrn\\compression.c",
        //"thirdparty\\opengr2\\libopengrn\\crc.c",
        // "thirdparty\\opengr2\\libopengrn\\darray.c",
        //"thirdparty\\opengr2\\libopengrn\\debug.c",
        //"thirdparty\\opengr2\\libopengrn\\elements.c",
        // "thirdparty\\opengr2\\libopengrn\\elements_parse.c",
        // "thirdparty\\opengr2\\libopengrn\\gr2.c",
        // "thirdparty\\opengr2\\libopengrn\\gr2_read.c",
        // "thirdparty\\opengr2\\libopengrn\\gr2_write.c",
        // "thirdparty\\opengr2\\libopengrn\\magic.c",
        // "thirdparty\\opengr2\\libopengrn\\oodle1.c.c",
        // "thirdparty\\opengr2\\libopengrn\\platform.c",
        // "thirdparty\\opengr2\\libopengrn\\typeinfo.c",
        // "thirdparty\\opengr2\\libopengrn\\virtual_pointer.c",
    ];

    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("include");
        //.flag("-Wno-unused-parameter")
        //.define("USE_ZLIB", None);
    
    build.compile(LIB_NAME);
}


/*
// TODO: We might not really need to link this way since we have configuration for building above
         but maybe we could provide a way of linking, decide if so 
fn main() {
   println!("cargo:rustc-link-lib={}", LIB_NAME);
}
*/


/*
# TODO: Check if we can use system_deps within Cargo
fn main() {
    system_deps::Config::new().probe().unwrap();
}
*/

