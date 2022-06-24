// TODO: Check if any case is necessary to be allowed/disallowed
// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]

use libc::*;

include!("./bindings/compression.rs");
include!("./bindings/crc.rs");
include!("./bindings/darray.rs");
include!("./bindings/debug.rs");
include!("./bindings/dllapi.rs");
include!("./bindings/elements.rs");
include!("./bindings/gr2.rs");
include!("./bindings/magic.rs");
include!("./bindings/oodle1.rs");
include!("./bindings/platform.rs");
include!("./bindings/structures.rs");
include!("./bindings/typeinfo.rs");
include!("./bindings/virtual_ptr.rs");
