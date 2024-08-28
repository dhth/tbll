# tbll

✨ Overview
---

`tbll` outputs data in tabular format.

💾 Installation
---

**homebrew**:

```sh
brew install dhth/tap/tbll
```

**cargo**:

```sh
cargo install --git https://github.com/dhth/tbll.git
```

Or get the binaries directly from a [release][1]. Read more about verifying the
authenticity of released artifacts [here](#-verifying-release-artifacts).

⚡️ Usage
---

### Help

```text
tbll outputs data in tabular format

Usage: tbll [OPTIONS]

Options:
  -r, --row <STRING:::STRING...>    Row elements
  -s, --read-stdin                  Whether to read row elements from stdin
  -d, --delimiter <STRING>          Delimiter to use [default: ,]
  -n, --num-cols <NUMBER>           Number of columns to output [default: 1]
      --headers <STRING,STRING...>  Command separated list of headers; overrides --num-cols when provided
      --style <STRING>              Border Style [default: sharp] [possible values: ascii, ascii-rounded, blank, dots, empty, extended, markdown, modern, modern-rounded, psql, re-structured-text, rounded, sharp]
      --left-pad <NUMBER>           Left padding for cells [default: 1]
      --right-pad <NUMBER>          Right padding for cells [default: 1]
  -h, --help                        Print help
```

### Basic Usage

```bash
cat << EOF | tbll -s --headers 'Movie,Year,Director,Genre'
The Matrix,1999,Lana Wachowski, Lilly Wachowski,Science Fiction
Fight Club,1999,David Fincher,Drama
Pulp Fiction,1994,Quentin Tarantino,Crime
The Shawshank Redemption,1994,Frank Darabont,Drama
Jurassic Park,1993,Steven Spielberg,Adventure
Forrest Gump,1994,Robert Zemeckis,Drama
EOF
```

```text
┌──────────────────────────┬──────┬───────────────────┬─────────────────┐
│ Movie                    │ Year │ Director          │ Genre           │
├──────────────────────────┼──────┼───────────────────┼─────────────────┤
│ The Matrix               │ 1999 │ Lana Wachowski    │ Lilly Wachowski │
│ Fight Club               │ 1999 │ David Fincher     │ Drama           │
│ Pulp Fiction             │ 1994 │ Quentin Tarantino │ Crime           │
│ The Shawshank Redemption │ 1994 │ Frank Darabont    │ Drama           │
│ Jurassic Park            │ 1993 │ Steven Spielberg  │ Adventure       │
│ Forrest Gump             │ 1994 │ Robert Zemeckis   │ Drama           │
└──────────────────────────┴──────┴───────────────────┴─────────────────┘
```

```bash
cat <<EOF | tbll -s -d ':::' \
    --headers 'col1,col2,col3' \
    --style psql \
    --left-pad 2 \
    --right-pad 2 \
    --row 'r1c1:::r1c2:::r1c3' \
    --row 'r2c1:::r2c2:::r2c3' \
    --row 'r3c1:::r3c2:::r3c3'
r4c1:::r4c2:::r4c3
r5c1:::r5c2:::r5c3
EOF
```

```text
  col1  |  col2  |  col3
--------+--------+--------
  r1c1  |  r1c2  |  r1c3
  r2c1  |  r2c2  |  r2c3
  r3c1  |  r3c2  |  r3c3
  r4c1  |  r4c2  |  r4c3
  r5c1  |  r5c2  |  r5c3
```

```bash
cat <<EOF | tbll -s -d ':::' --headers 'attribute,value'
event:::COMMIT_PUSHED
system:::tbll
env:::prod
commit:::$(git rev-parse --short HEAD 2>/dev/null)
message:::$(git log -1 --pretty=format:'%B' 2>/dev/null)
stat:::$(git diff HEAD~1..HEAD --shortstat 2>/dev/null)
author:::$(git log -1 --pretty=format:'%ae' 2>/dev/null)
EOF
```

```text
┌───────────┬──────────────────────────────────────────────────┐
│ attribute │ value                                            │
├───────────┼──────────────────────────────────────────────────┤
│ event     │ COMMIT_PUSHED                                    │
│ system    │ tbll                                             │
│ env       │ prod                                             │
│ commit    │ fcb1cdc                                          │
│ message   │ docs: add more examples                          │
│ stat      │ 2 files changed, 15 insertions(+), 1 deletion(-) │
│ author    │ 13575379+dhth@users.noreply.github.com           │
└───────────┴──────────────────────────────────────────────────┘
```

🔐 Verifying release artifacts
---

In case you get the `tbll` binary directly from a [release][1], you may want to
verify its authenticity. Checksums are applied to all released artifacts, and
the resulting checksum file is attested using [Github Attestations][2].

Steps to verify (replace `A.B.C` in the commands below with the version you
want):

1. Download the sha256 checksum file for your platform from the release:

   ```shell
   curl -sSLO https://github.com/dhth/tbll/releases/download/vA.B.C/tbll-x86_64-unknown-linux-gnu.tar.xz.sha256
   ```

2. Verify the integrity of the checksum file using [gh][3].

   ```shell
   gh attestation verify tbll-x86_64-unknown-linux-gnu.tar.xz.sha256 --repo dhth/tbll
   ```

3. Download the compressed archive you want, and validate its checksum:

   ```shell
   curl -sSLO https://github.com/dhth/tbll/releases/download/vA.B.C/tbll-x86_64-unknown-linux-gnu.tar.xz
   sha256sum --ignore-missing -c tbll-x86_64-unknown-linux-gnu.tar.xz.sha256
   ```

3. If checksum validation goes through, uncompress the archive:

   ```shell
   tar -xzf tbll-x86_64-unknown-linux-gnu.tar.xz
   ./tbll
   # profit!
   ```

[1]: https://github.com/dhth/tbll/releases
[2]: https://github.blog/news-insights/product-news/introducing-artifact-attestations-now-in-public-beta/
[3]: https://github.com/cli/cli
