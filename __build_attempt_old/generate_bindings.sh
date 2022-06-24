bindings_directory="src/bindings/"

library_include_path="thirdparty/opengr2/libopengrn"
library_include_files=(
    "compression.h"
    "crc.h"
    "darray.h"
    "debug.h"
    "dllapi.h"
    "elements.h"
    "gr2.h"
    "magic.h"
    "oodle1.h"
    "platform.h"
    "structures.h"
    "typeinfo.h"
    "virtual_ptr.h"
)

clang_arguments=""

rm -rf $bindings_directory
mkdir $bindings_directory

for include in "${library_include_files[@]}"; do
    filename=$(basename $include .h)
    arguments=(
        --whitelist-function "^$filename_.*"
        --whitelist-var "^$filename_.*"
        --no-doc-comments
    )
    bindgen $library_include_path/$include -o $bindings_directory/$filename.rs ${arguments[@]} -- $clang_arguments
done

