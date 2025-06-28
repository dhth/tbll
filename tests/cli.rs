mod common;

use common::base_command;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn shows_help() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.arg("--help");

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    tbll outputs data in tabular format

    Usage: tbll [OPTIONS]

    Options:
      -p, --input-path <STRING>           Input file path; tbll will read from stdin if this is not provided
      -d, --delimiter <STRING>            Delimiter to use [default: ,]
          --headers <STRING,STRING...>    Command separated list of headers
      -c, --cols <NUMBER,NUMBER...>       Indices of columns (starting from zero) to display
      -C, --skip-cols <NUMBER,NUMBER...>  Indices of columns (starting from zero) to skip
      -s, --style <STRING>                Border Style [default: sharp] [possible values: ascii, ascii-rounded, blank, dots, empty, extended, markdown, modern, modern-rounded, psql, re-structured-text, rounded, sharp]
      -l, --left-pad <NUMBER>             Left padding for cells [default: 1]
      -r, --right-pad <NUMBER>            Right padding for cells [default: 1]
      -h, --help                          Print help

    ----- stderr -----
    ");
}

#[test]
fn works_for_input_file() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.arg("-p=tests/data/input-1.txt");

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    ┌──────────────────────────┬──────┬────────────────────────┬─────────────────┐
    │ Movie                    │ Year │ Director               │ Genre           │
    ├──────────────────────────┼──────┼────────────────────────┼─────────────────┤
    │ The Matrix               │ 1999 │ Lana & Lilly Wachowski │ Science Fiction │
    │ Pulp Fiction             │ 1994 │ Quentin Tarantino      │ Crime           │
    │ The Shawshank Redemption │ 1994 │ Frank Darabont         │ Drama           │
    └──────────────────────────┴──────┴────────────────────────┴─────────────────┘

    ----- stderr -----
    ");
}

#[test]
fn works_with_custom_delimiter() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.arg("-p=tests/data/input-3.txt");
    cmd.arg("-d=|");

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    ┌──────────────────────────┬──────┬────────────────────────┬─────────────────┐
    │ Movie                    │ Year │ Director               │ Genre           │
    ├──────────────────────────┼──────┼────────────────────────┼─────────────────┤
    │ The Matrix               │ 1999 │ Lana & Lilly Wachowski │ Science Fiction │
    │ Pulp Fiction             │ 1994 │ Quentin Tarantino      │ Crime           │
    │ The Shawshank Redemption │ 1994 │ Frank Darabont         │ Drama           │
    └──────────────────────────┴──────┴────────────────────────┴─────────────────┘

    ----- stderr -----
    ");
}

#[test]
fn works_with_empty_cells() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.arg("-p=tests/data/input-4.txt");

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    ┌──────────────────────────┬──────┬────────────────────────┬─────────────────┐
    │ Movie                    │ Year │ Director               │ Genre           │
    ├──────────────────────────┼──────┼────────────────────────┼─────────────────┤
    │ The Matrix               │ 1999 │ Lana & Lilly Wachowski │ Science Fiction │
    │ Pulp Fiction             │      │ Quentin Tarantino      │ Crime           │
    │ The Shawshank Redemption │ 1994 │                        │ Drama           │
    └──────────────────────────┴──────┴────────────────────────┴─────────────────┘

    ----- stderr -----
    ");
}

#[test]
fn using_headers_works() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.arg("-p=tests/data/input-2.txt");
    cmd.arg("--headers=Movie,Year,Director,Genre");

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    ┌──────────────────────────┬──────┬────────────────────────┬─────────────────┐
    │ Movie                    │ Year │ Director               │ Genre           │
    ├──────────────────────────┼──────┼────────────────────────┼─────────────────┤
    │ The Matrix               │ 1999 │ Lana & Lilly Wachowski │ Science Fiction │
    │ Pulp Fiction             │ 1994 │ Quentin Tarantino      │ Crime           │
    │ The Shawshank Redemption │ 1994 │ Frank Darabont         │ Drama           │
    └──────────────────────────┴──────┴────────────────────────┴─────────────────┘

    ----- stderr -----
    ");
}

#[test]
fn using_custom_style_works() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("--style=ascii");

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    +--------------------------+------+------------------------+-----------------+
    | Movie                    | Year | Director               | Genre           |
    +--------------------------+------+------------------------+-----------------+
    | The Matrix               | 1999 | Lana & Lilly Wachowski | Science Fiction |
    +--------------------------+------+------------------------+-----------------+
    | Pulp Fiction             | 1994 | Quentin Tarantino      | Crime           |
    +--------------------------+------+------------------------+-----------------+
    | The Shawshank Redemption | 1994 | Frank Darabont         | Drama           |
    +--------------------------+------+------------------------+-----------------+

    ----- stderr -----
    ");
}

#[test]
fn using_left_pad_works() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("--left-pad=4");

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    ┌─────────────────────────────┬─────────┬───────────────────────────┬────────────────────┐
    │    Movie                    │    Year │    Director               │    Genre           │
    ├─────────────────────────────┼─────────┼───────────────────────────┼────────────────────┤
    │    The Matrix               │    1999 │    Lana & Lilly Wachowski │    Science Fiction │
    │    Pulp Fiction             │    1994 │    Quentin Tarantino      │    Crime           │
    │    The Shawshank Redemption │    1994 │    Frank Darabont         │    Drama           │
    └─────────────────────────────┴─────────┴───────────────────────────┴────────────────────┘

    ----- stderr -----
    ");
}

#[test]
fn using_right_pad_works() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("--right-pad=4");

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    ┌─────────────────────────────┬─────────┬───────────────────────────┬────────────────────┐
    │ Movie                       │ Year    │ Director                  │ Genre              │
    ├─────────────────────────────┼─────────┼───────────────────────────┼────────────────────┤
    │ The Matrix                  │ 1999    │ Lana & Lilly Wachowski    │ Science Fiction    │
    │ Pulp Fiction                │ 1994    │ Quentin Tarantino         │ Crime              │
    │ The Shawshank Redemption    │ 1994    │ Frank Darabont            │ Drama              │
    └─────────────────────────────┴─────────┴───────────────────────────┴────────────────────┘

    ----- stderr -----
    ");
}

#[test]
fn reading_input_where_row_item_contains_delimiter_works() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.arg("-p=tests/data/input-5.txt");

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    ┌────────────────────────────────┬──────┬───────────────────────┬──────────────────┐
    │ Movie                          │ Year │ Director              │ Genre            │
    ├────────────────────────────────┼──────┼───────────────────────┼──────────────────┤
    │ The Matrix                     │ 1999 │ Lana, Lilly Wachowski │ Science Fiction  │
    │ Pulp Fiction                   │ 1994 │ Quentin Tarantino     │ Crime            │
    │ The Shawshank Redemption       │ 1994 │ Frank Darabont        │ Drama            │
    │ The Good, the Bad and the Ugly │ 1967 │ Sergio Leone          │ Spagetti Western │
    └────────────────────────────────┴──────┴───────────────────────┴──────────────────┘

    ----- stderr -----
    ");
}

#[test]
fn selecting_specific_columns_works() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args(["-p=tests/data/input-1.txt", "-c=0,2"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    ┌──────────────────────────┬────────────────────────┐
    │ Movie                    │ Director               │
    ├──────────────────────────┼────────────────────────┤
    │ The Matrix               │ Lana & Lilly Wachowski │
    │ Pulp Fiction             │ Quentin Tarantino      │
    │ The Shawshank Redemption │ Frank Darabont         │
    └──────────────────────────┴────────────────────────┘

    ----- stderr -----
    ");
}

#[test]
fn skipping_specific_columns_works() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args(["-p=tests/data/input-1.txt", "-C=1,2"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    ┌──────────────────────────┬─────────────────┐
    │ Movie                    │ Genre           │
    ├──────────────────────────┼─────────────────┤
    │ The Matrix               │ Science Fiction │
    │ Pulp Fiction             │ Crime           │
    │ The Shawshank Redemption │ Drama           │
    └──────────────────────────┴─────────────────┘

    ----- stderr -----
    ");
}

//------------//
//  FAILURES  //
//------------//

#[test]
fn fails_if_more_than_one_source_is_provided() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-s");

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: a value is required for '--style <STRING>' but none was supplied
      [possible values: ascii, ascii-rounded, blank, dots, empty, extended, markdown, modern, modern-rounded, psql, re-structured-text, rounded, sharp]

    For more information, try '--help'.
    ");
}

#[test]
fn fails_if_input_file_is_non_existent() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.arg("-p=tests/data/nonexistent.txt");

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: No such file or directory (os error 2)
    ");
}

#[test]
fn fails_if_style_is_not_supported() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.arg("-p=tests/data/nonexistent.txt");
    cmd.arg("--style=blah");

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: invalid value 'blah' for '--style <STRING>'
      [possible values: ascii, ascii-rounded, blank, dots, empty, extended, markdown, modern, modern-rounded, psql, re-structured-text, rounded, sharp]

      tip: a similar value exists: 'blank'

    For more information, try '--help'.
    ");
}

#[test]
fn fails_if_left_pad_is_not_a_number() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.arg("-p=tests/data/nonexistent.txt");
    cmd.arg("--left-pad=blah");

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: invalid value 'blah' for '--left-pad <NUMBER>': invalid digit found in string

    For more information, try '--help'.
    ");
}

#[test]
fn fails_if_right_pad_is_not_a_number() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.arg("-p=tests/data/nonexistent.txt");
    cmd.arg("--left-pad=blah");

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: invalid value 'blah' for '--left-pad <NUMBER>': invalid digit found in string

    For more information, try '--help'.
    ");
}

#[test]
fn fails_if_both_cols_and_skip_cols_are_provided() {
    // GIVEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args(["-p=tests/data/input-1.txt", "-c=0,3", "-C=1,2"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: --cols and --skip-cols cannot be used at the same time
    ");
}
