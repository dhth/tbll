#/usr/bin/env bash

cat <<EOF | ./target/release/tbll \
    -s \
    --headers 'attribute,value'
event:::COMMIT_PUSHED
system:::tbll
env:::prod
commit:::$(git rev-parse --short HEAD 2>/dev/null)
message:::$(git log -1 --pretty=format:'%B' 2>/dev/null)
stat:::$(git diff HEAD~1..HEAD --shortstat 2>/dev/null)
author:::$(git log -1 --pretty=format:'%ae' 2>/dev/null)
EOF
