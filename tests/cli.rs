use assert_cmd::Command;
use pretty_assertions::assert_eq;

// SUCCESSES
#[test]
fn shows_help() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--help");
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    assert!(stdout.contains("tbll outputs data in tabular format"));
}

#[test]
fn works_for_input_file() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-n=4");
    let output = cmd.output().expect("running command failed");
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }

    // THEN
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
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-3.txt");
    cmd.arg("-d=|");
    cmd.arg("-n=4");
    let output = cmd.output().expect("running command failed");
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }

    // THEN
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
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-4.txt");
    cmd.arg("-n=4");
    let output = cmd.output().expect("running command failed");
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }

    // THEN
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
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-2.txt");
    cmd.arg("--headers=Movie,Year,Director,Genre");
    let output = cmd.output().expect("running command failed");
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }

    // THEN
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
fn using_num_cols_works() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-n=2");
    let output = cmd.output().expect("running command failed");
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"
┌──────────────────────────┬──────┐
│ Movie                    │ Year │
├──────────────────────────┼──────┤
│ The Matrix               │ 1999 │
│ Pulp Fiction             │ 1994 │
│ The Shawshank Redemption │ 1994 │
└──────────────────────────┴──────┘
"#;
    assert_eq!(stdout, expected.trim_start());
}

#[test]
fn using_custom_style_works() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-n=4");
    cmd.arg("--style=ascii");
    let output = cmd.output().expect("running command failed");
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }

    // THEN
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
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-n=4");
    cmd.arg("--left-pad=4");
    let output = cmd.output().expect("running command failed");
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }

    // THEN
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
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-n=4");
    cmd.arg("--right-pad=4");
    let output = cmd.output().expect("running command failed");
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }

    // THEN
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
fn providing_rows_works() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-2.txt");
    cmd.arg("-r=Movie,Year,Director,Genre");
    cmd.arg("-r=Jurassic Park,1993,Steven Spielberg,Adventure");
    cmd.arg("-n=4");
    let output = cmd.output().expect("running command failed");
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).expect("invalid utf-8 stderr");
        println!("stderr: \n{}", stderr);
    }

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    let expected = r#"
┌──────────────────────────┬──────┬────────────────────────┬─────────────────┐
│ Movie                    │ Year │ Director               │ Genre           │
├──────────────────────────┼──────┼────────────────────────┼─────────────────┤
│ Jurassic Park            │ 1993 │ Steven Spielberg       │ Adventure       │
│ The Matrix               │ 1999 │ Lana & Lilly Wachowski │ Science Fiction │
│ Pulp Fiction             │ 1994 │ Quentin Tarantino      │ Crime           │
│ The Shawshank Redemption │ 1994 │ Frank Darabont         │ Drama           │
└──────────────────────────┴──────┴────────────────────────┴─────────────────┘
"#;
    assert_eq!(stdout, expected.trim_start());
}

// FAILURES
#[test]
fn fails_if_no_source_is_provided() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success());
}

#[test]
fn fails_if_more_than_one_source_is_provided() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/input-1.txt");
    cmd.arg("-s");
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
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/nonexistent.txt");
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
fn fails_if_num_cols_is_not_a_number() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/nonexistent.txt");
    cmd.arg("-n=blah");
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success());
}

#[test]
fn fails_if_style_is_not_supported() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/nonexistent.txt");
    cmd.arg("--style=blah");
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
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/nonexistent.txt");
    cmd.arg("--left-pad=blah");
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
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("-p=tests/data/nonexistent.txt");
    cmd.arg("--left-pad=blah");
    let output = cmd.output().expect("running command failed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success());
}
