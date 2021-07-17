use assert_cmd::Command;
use std::fs::remove_file;

fn do_test(file_name: &str) {
    let mut cmd = Command::cargo_bin("lzd").unwrap();
    cmd.arg(&file_name).arg("-f").arg("-t").assert().success();
    remove_file(format!("{}.lzd", file_name)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const CANTRBRY_DIR: &str = "cantrbry";

    #[test]
    #[ignore]
    fn alice29() {
        do_test(&format!("{}/alice29.txt", CANTRBRY_DIR));
    }

    #[test]
    #[ignore]
    fn asyoulik() {
        do_test(&format!("{}/asyoulik.txt", CANTRBRY_DIR));
    }

    #[test]
    #[ignore]
    fn cp() {
        do_test(&format!("{}/cp.html", CANTRBRY_DIR));
    }

    #[test]
    #[ignore]
    fn fields() {
        do_test(&format!("{}/fields.c", CANTRBRY_DIR));
    }

    #[test]
    #[ignore]
    fn grammar() {
        do_test(&format!("{}/grammar.lsp", CANTRBRY_DIR));
    }

    #[test]
    #[ignore]
    fn kennedy() {
        do_test(&format!("{}/kennedy.xls", CANTRBRY_DIR));
    }

    #[test]
    #[ignore]
    fn lcet10() {
        do_test(&format!("{}/lcet10.txt", CANTRBRY_DIR));
    }

    #[test]
    #[ignore]
    fn plrabn12() {
        do_test(&format!("{}/plrabn12.txt", CANTRBRY_DIR));
    }

    #[test]
    #[ignore]
    fn ptt5() {
        do_test(&format!("{}/ptt5", CANTRBRY_DIR));
    }

    #[test]
    #[ignore]
    fn sum() {
        do_test(&format!("{}/sum", CANTRBRY_DIR));
    }

    #[test]
    #[ignore]
    fn xargs() {
        do_test(&format!("{}/xargs.1", CANTRBRY_DIR));
    }
}
