use assert_cmd::Command;
use predicates::str::contains;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn shows_help() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--help");

    // WHEN
    // THEN
    cmd.assert()
        .success()
        .stdout(contains("tbll outputs data in tabular format"));
}

#[test]
fn works_for_input_file() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");

    // WHEN
    // THEN
    cmd.assert().success().stdout(contains(
        r#"
┌──────────────────────────┬──────┬────────────────────────┬─────────────────┐
│ Movie                    │ Year │ Director               │ Genre           │
├──────────────────────────┼──────┼────────────────────────┼─────────────────┤
│ The Matrix               │ 1999 │ Lana & Lilly Wachowski │ Science Fiction │
│ Pulp Fiction             │ 1994 │ Quentin Tarantino      │ Crime           │
│ The Shawshank Redemption │ 1994 │ Frank Darabont         │ Drama           │
└──────────────────────────┴──────┴────────────────────────┴─────────────────┘
"#
        .trim(),
    ));
}

#[test]
fn works_with_custom_delimiter() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-3.txt");
    cmd.arg("-d=|");

    // WHEN
    // THEN
    cmd.assert().success().stdout(contains(
        r#"
┌──────────────────────────┬──────┬────────────────────────┬─────────────────┐
│ Movie                    │ Year │ Director               │ Genre           │
├──────────────────────────┼──────┼────────────────────────┼─────────────────┤
│ The Matrix               │ 1999 │ Lana & Lilly Wachowski │ Science Fiction │
│ Pulp Fiction             │ 1994 │ Quentin Tarantino      │ Crime           │
│ The Shawshank Redemption │ 1994 │ Frank Darabont         │ Drama           │
└──────────────────────────┴──────┴────────────────────────┴─────────────────┘
"#
        .trim(),
    ));
}

#[test]
fn works_with_empty_cells() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-4.txt");

    // WHEN
    // THEN
    cmd.assert().success().stdout(contains(
        r#"
┌──────────────────────────┬──────┬────────────────────────┬─────────────────┐
│ Movie                    │ Year │ Director               │ Genre           │
├──────────────────────────┼──────┼────────────────────────┼─────────────────┤
│ The Matrix               │ 1999 │ Lana & Lilly Wachowski │ Science Fiction │
│ Pulp Fiction             │      │ Quentin Tarantino      │ Crime           │
│ The Shawshank Redemption │ 1994 │                        │ Drama           │
└──────────────────────────┴──────┴────────────────────────┴─────────────────┘
"#
        .trim(),
    ));
}

#[test]
fn using_headers_works() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-2.txt");
    cmd.arg("--headers=Movie,Year,Director,Genre");

    // WHEN
    // THEN
    cmd.assert().success().stdout(contains(
        r#"
┌──────────────────────────┬──────┬────────────────────────┬─────────────────┐
│ Movie                    │ Year │ Director               │ Genre           │
├──────────────────────────┼──────┼────────────────────────┼─────────────────┤
│ The Matrix               │ 1999 │ Lana & Lilly Wachowski │ Science Fiction │
│ Pulp Fiction             │ 1994 │ Quentin Tarantino      │ Crime           │
│ The Shawshank Redemption │ 1994 │ Frank Darabont         │ Drama           │
└──────────────────────────┴──────┴────────────────────────┴─────────────────┘
"#
        .trim(),
    ));
}

#[test]
fn using_custom_style_works() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("--style=ascii");

    // WHEN
    // THEN
    cmd.assert().success().stdout(contains(
        r#"
+--------------------------+------+------------------------+-----------------+
| Movie                    | Year | Director               | Genre           |
+--------------------------+------+------------------------+-----------------+
| The Matrix               | 1999 | Lana & Lilly Wachowski | Science Fiction |
+--------------------------+------+------------------------+-----------------+
| Pulp Fiction             | 1994 | Quentin Tarantino      | Crime           |
+--------------------------+------+------------------------+-----------------+
| The Shawshank Redemption | 1994 | Frank Darabont         | Drama           |
+--------------------------+------+------------------------+-----------------+
"#
        .trim(),
    ));
}

#[test]
fn using_left_pad_works() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("--left-pad=4");

    // WHEN
    // THEN
    cmd.assert().success().stdout(contains(
        r#"
┌─────────────────────────────┬─────────┬───────────────────────────┬────────────────────┐
│    Movie                    │    Year │    Director               │    Genre           │
├─────────────────────────────┼─────────┼───────────────────────────┼────────────────────┤
│    The Matrix               │    1999 │    Lana & Lilly Wachowski │    Science Fiction │
│    Pulp Fiction             │    1994 │    Quentin Tarantino      │    Crime           │
│    The Shawshank Redemption │    1994 │    Frank Darabont         │    Drama           │
└─────────────────────────────┴─────────┴───────────────────────────┴────────────────────┘
"#
        .trim(),
    ));
}

#[test]
fn using_right_pad_works() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("--right-pad=4");

    // WHEN
    // THEN
    cmd.assert().success().stdout(contains(
        r#"
┌─────────────────────────────┬─────────┬───────────────────────────┬────────────────────┐
│ Movie                       │ Year    │ Director                  │ Genre              │
├─────────────────────────────┼─────────┼───────────────────────────┼────────────────────┤
│ The Matrix                  │ 1999    │ Lana & Lilly Wachowski    │ Science Fiction    │
│ Pulp Fiction                │ 1994    │ Quentin Tarantino         │ Crime              │
│ The Shawshank Redemption    │ 1994    │ Frank Darabont            │ Drama              │
└─────────────────────────────┴─────────┴───────────────────────────┴────────────────────┘
"#
        .trim(),
    ));
}

#[test]
fn reading_input_where_row_item_contains_delimiter_works() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-5.txt");

    // WHEN
    // THEN
    cmd.assert().success().stdout(contains(
        r#"
┌────────────────────────────────┬──────┬───────────────────────┬──────────────────┐
│ Movie                          │ Year │ Director              │ Genre            │
├────────────────────────────────┼──────┼───────────────────────┼──────────────────┤
│ The Matrix                     │ 1999 │ Lana, Lilly Wachowski │ Science Fiction  │
│ Pulp Fiction                   │ 1994 │ Quentin Tarantino     │ Crime            │
│ The Shawshank Redemption       │ 1994 │ Frank Darabont        │ Drama            │
│ The Good, the Bad and the Ugly │ 1967 │ Sergio Leone          │ Spagetti Western │
└────────────────────────────────┴──────┴───────────────────────┴──────────────────┘
"#
        .trim(),
    ));
}

#[test]
fn selecting_specific_columns_works() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(["-p=tests/data/input-1.txt", "-c=0,2"]);

    // WHEN
    // THEN
    cmd.assert().success().stdout(contains(
        r#"
┌──────────────────────────┬────────────────────────┐
│ Movie                    │ Director               │
├──────────────────────────┼────────────────────────┤
│ The Matrix               │ Lana & Lilly Wachowski │
│ Pulp Fiction             │ Quentin Tarantino      │
│ The Shawshank Redemption │ Frank Darabont         │
└──────────────────────────┴────────────────────────┘
"#
        .trim(),
    ));
}

//------------//
//  FAILURES  //
//------------//

#[test]
fn fails_if_more_than_one_source_is_provided() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-s");

    // WHEN
    // THEN
    cmd.assert().failure();
}

#[test]
fn fails_if_input_file_is_non_existent() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/nonexistent.txt");

    // WHEN
    // THEN
    cmd.assert()
        .failure()
        .stderr(contains("No such file or directory"));
}

#[test]
fn fails_if_style_is_not_supported() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/nonexistent.txt");
    cmd.arg("--style=blah");

    // WHEN
    // THEN
    cmd.assert().failure();
}

#[test]
fn fails_if_left_pad_is_not_a_number() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/nonexistent.txt");
    cmd.arg("--left-pad=blah");

    // WHEN
    // THEN
    cmd.assert().failure();
}

#[test]
fn fails_if_right_pad_is_not_a_number() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/nonexistent.txt");
    cmd.arg("--left-pad=blah");

    // WHEN
    // THEN
    cmd.assert().failure();
}
