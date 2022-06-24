bindings_directory="src/bindings/"
bindgen_arguments=""
# TODO: Fiddle with arguments to make a generalization of output that ignores system/standard bindings
# bindgen_arguments = "'.*' --whitelist-function '^foo_.*' --whitelist-var '^FOO_.*'"

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

rm -rf $bindings_directory
mkdir $bindings_directory

for include in "${library_include_files[@]}"; do
    filename=$(basename ${include} .h)
    bindgen $library_include_path/$include -o $bindings_directory/$filename.rs $bindgen_arguments
done

