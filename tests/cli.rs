use assert_cmd::Command;
use pretty_assertions::assert_eq;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn shows_help() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--help");

    // WHEN
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    assert!(stdout.contains("tbll outputs data in tabular format"));
}

#[test]
fn works_for_input_file() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");

    // WHEN
    let output = cmd.output().expect("running command failed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"
┌──────────────────────────┬──────┬────────────────────────┬─────────────────┐
│ Movie                    │ Year │ Director               │ Genre           │
├──────────────────────────┼──────┼────────────────────────┼─────────────────┤
│ The Matrix               │ 1999 │ Lana & Lilly Wachowski │ Science Fiction │
│ Pulp Fiction             │ 1994 │ Quentin Tarantino      │ Crime           │
│ The Shawshank Redemption │ 1994 │ Frank Darabont         │ Drama           │
└──────────────────────────┴──────┴────────────────────────┴─────────────────┘
"#;
    assert_eq!(stdout, expected.trim_start());
}

#[test]
fn works_with_custom_delimiter() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-3.txt");
    cmd.arg("-d=|");

    // WHEN
    let output = cmd.output().expect("running command failed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"
┌──────────────────────────┬──────┬────────────────────────┬─────────────────┐
│ Movie                    │ Year │ Director               │ Genre           │
├──────────────────────────┼──────┼────────────────────────┼─────────────────┤
│ The Matrix               │ 1999 │ Lana & Lilly Wachowski │ Science Fiction │
│ Pulp Fiction             │ 1994 │ Quentin Tarantino      │ Crime           │
│ The Shawshank Redemption │ 1994 │ Frank Darabont         │ Drama           │
└──────────────────────────┴──────┴────────────────────────┴─────────────────┘
"#;
    assert_eq!(stdout, expected.trim_start());
}

#[test]
fn works_with_empty_cells() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-4.txt");

    // WHEN
    let output = cmd.output().expect("running command failed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"
┌──────────────────────────┬──────┬────────────────────────┬─────────────────┐
│ Movie                    │ Year │ Director               │ Genre           │
├──────────────────────────┼──────┼────────────────────────┼─────────────────┤
│ The Matrix               │ 1999 │ Lana & Lilly Wachowski │ Science Fiction │
│ Pulp Fiction             │      │ Quentin Tarantino      │ Crime           │
│ The Shawshank Redemption │ 1994 │                        │ Drama           │
└──────────────────────────┴──────┴────────────────────────┴─────────────────┘
"#;
    assert_eq!(stdout, expected.trim_start());
}

#[test]
fn using_headers_works() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-2.txt");
    cmd.arg("--headers=Movie,Year,Director,Genre");

    // WHEN
    let output = cmd.output().expect("running command failed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"
┌──────────────────────────┬──────┬────────────────────────┬─────────────────┐
│ Movie                    │ Year │ Director               │ Genre           │
├──────────────────────────┼──────┼────────────────────────┼─────────────────┤
│ The Matrix               │ 1999 │ Lana & Lilly Wachowski │ Science Fiction │
│ Pulp Fiction             │ 1994 │ Quentin Tarantino      │ Crime           │
│ The Shawshank Redemption │ 1994 │ Frank Darabont         │ Drama           │
└──────────────────────────┴──────┴────────────────────────┴─────────────────┘
"#;
    assert_eq!(stdout, expected.trim_start());
}

#[test]
fn using_custom_style_works() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("--style=ascii");

    // WHEN
    let output = cmd.output().expect("running command failed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"
+--------------------------+------+------------------------+-----------------+
| Movie                    | Year | Director               | Genre           |
+--------------------------+------+------------------------+-----------------+
| The Matrix               | 1999 | Lana & Lilly Wachowski | Science Fiction |
+--------------------------+------+------------------------+-----------------+
| Pulp Fiction             | 1994 | Quentin Tarantino      | Crime           |
+--------------------------+------+------------------------+-----------------+
| The Shawshank Redemption | 1994 | Frank Darabont         | Drama           |
+--------------------------+------+------------------------+-----------------+
"#;
    assert_eq!(stdout, expected.trim_start());
}

#[test]
fn using_left_pad_works() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("--left-pad=4");

    // WHEN
    let output = cmd.output().expect("running command failed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"
┌─────────────────────────────┬─────────┬───────────────────────────┬────────────────────┐
│    Movie                    │    Year │    Director               │    Genre           │
├─────────────────────────────┼─────────┼───────────────────────────┼────────────────────┤
│    The Matrix               │    1999 │    Lana & Lilly Wachowski │    Science Fiction │
│    Pulp Fiction             │    1994 │    Quentin Tarantino      │    Crime           │
│    The Shawshank Redemption │    1994 │    Frank Darabont         │    Drama           │
└─────────────────────────────┴─────────┴───────────────────────────┴────────────────────┘
"#;
    assert_eq!(stdout, expected.trim_start());
}

#[test]
fn using_right_pad_works() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("--right-pad=4");

    // WHEN
    let output = cmd.output().expect("running command failed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"
┌─────────────────────────────┬─────────┬───────────────────────────┬────────────────────┐
│ Movie                       │ Year    │ Director                  │ Genre              │
├─────────────────────────────┼─────────┼───────────────────────────┼────────────────────┤
│ The Matrix                  │ 1999    │ Lana & Lilly Wachowski    │ Science Fiction    │
│ Pulp Fiction                │ 1994    │ Quentin Tarantino         │ Crime              │
│ The Shawshank Redemption    │ 1994    │ Frank Darabont            │ Drama              │
└─────────────────────────────┴─────────┴───────────────────────────┴────────────────────┘
"#;
    assert_eq!(stdout, expected.trim_start());
}

#[test]
fn reading_input_where_row_item_contains_delimiter_works() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-5.txt");

    // WHEN
    let output = cmd.output().expect("running command failed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"
┌────────────────────────────────┬──────┬───────────────────────┬──────────────────┐
│ Movie                          │ Year │ Director              │ Genre            │
├────────────────────────────────┼──────┼───────────────────────┼──────────────────┤
│ The Matrix                     │ 1999 │ Lana, Lilly Wachowski │ Science Fiction  │
│ Pulp Fiction                   │ 1994 │ Quentin Tarantino     │ Crime            │
│ The Shawshank Redemption       │ 1994 │ Frank Darabont        │ Drama            │
│ The Good, the Bad and the Ugly │ 1967 │ Sergio Leone          │ Spagetti Western │
└────────────────────────────────┴──────┴───────────────────────┴──────────────────┘
"#;
    assert_eq!(stdout, expected.trim_start());
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
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success());
}

#[test]
fn fails_if_input_file_is_non_existent() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/nonexistent.txt");

    // WHEN
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
    assert!(stderr.contains("No such file or directory"));
}

#[test]
fn fails_if_style_is_not_supported() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/nonexistent.txt");
    cmd.arg("--style=blah");

    // WHEN
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success());
}

#[test]
fn fails_if_left_pad_is_not_a_number() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/nonexistent.txt");
    cmd.arg("--left-pad=blah");

    // WHEN
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success());
}

#[test]
fn fails_if_right_pad_is_not_a_number() {
    // GIVEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/nonexistent.txt");
    cmd.arg("--left-pad=blah");

    // WHEN
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success());
}
