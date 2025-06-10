#!/bin/bash

echo "Moving build files to appropriate directories for the Rust server."

mv dist/assets/*.css ../../static/assets/
mv dist/assets/*.js ../../static/assets/
mv dist/index.html ../../static/sc-index.html
