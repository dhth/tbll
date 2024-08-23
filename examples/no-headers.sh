#/usr/bin/env bash

cat <<EOF | tbll \
    -s \
    -n 3 \
    -r 'col1:::col2:::col3' \
    -r 'r1c1:::r1c2:::r1c3' \
    -r 'r2c1:::r2c2:::r2c3' \
    -r 'r3c1:::r3c2:::r3c3'
r4c1:::r4c2:::r4c3
r5c1:::r5c2:::r5c3
EOF
