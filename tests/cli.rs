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
    fn alice29() {
        do_test(&format!("{}/alice29.txt", CANTRBRY_DIR));
    }

    #[test]
    fn asyoulik() {
        do_test(&format!("{}/asyoulik.txt", CANTRBRY_DIR));
    }

    #[test]
    fn cp() {
        do_test(&format!("{}/cp.html", CANTRBRY_DIR));
    }

    #[test]
    fn fields() {
        do_test(&format!("{}/fields.c", CANTRBRY_DIR));
    }

    #[test]
    fn grammar() {
        do_test(&format!("{}/grammar.lsp", CANTRBRY_DIR));
    }

    #[test]
    fn kennedy() {
        do_test(&format!("{}/kennedy.xls", CANTRBRY_DIR));
    }

    #[test]
    fn lcet10() {
        do_test(&format!("{}/lcet10.txt", CANTRBRY_DIR));
    }

    #[test]
    fn plrabn12() {
        do_test(&format!("{}/plrabn12.txt", CANTRBRY_DIR));
    }

    #[test]
    fn ptt5() {
        do_test(&format!("{}/ptt5", CANTRBRY_DIR));
    }

    #[test]
    fn sum() {
        do_test(&format!("{}/sum", CANTRBRY_DIR));
    }

    #[test]
    fn xargs() {
        do_test(&format!("{}/xargs.1", CANTRBRY_DIR));
    }
}
