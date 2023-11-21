#!/bin/bash
rustdoc src/lib.rs --crate-name indexable_str
echo "<meta http-equiv=\"refresh\" content=\"0; url=doc/indexable_str/index.html\">" > ./index.html