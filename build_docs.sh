#!/bin/bash
rustdoc src/lib.rs --crate-name indexable_str
echo "<meta http-equiv=\"refresh\" content=\"0; url=indexable_str/index.html\">" > doc/index.html