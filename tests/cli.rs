use assert_cmd::Command;
use std::fs::{remove_file, File};
use std::io::Read;

fn read_text(file_name: &str) -> Vec<u8> {
    let mut text: Vec<u8> = Vec::new();
    {
        let mut file = File::open(file_name).unwrap();
        let _ = file.read_to_end(&mut text).unwrap();
    }
    text
}

fn do_work(file_name: &str) {
    let lzd_file_name = format!("{}.lzd", file_name);
    let unlzd_file_name = format!("{}.unlzd", file_name);

    {
        let mut cmd = Command::cargo_bin("lzd").unwrap();
        cmd.arg(&file_name)
            .arg("-o")
            .arg(&lzd_file_name)
            .assert()
            .success();
    }

    {
        let mut cmd = Command::cargo_bin("unlzd").unwrap();
        cmd.arg(&lzd_file_name)
            .arg("-o")
            .arg(&unlzd_file_name)
            .assert()
            .success();
    }

    let text1 = read_text(&file_name);
    let text2 = read_text(&unlzd_file_name);

    assert_eq!(text1.len(), text2.len());

    for i in 0..text1.len() {
        assert_eq!(text1[i], text2[i]);
    }

    remove_file(lzd_file_name).unwrap();
    remove_file(unlzd_file_name).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const CANTRBRY_DIR: &str = "cantrbry";

    #[test]
    fn alice29() {
        do_work(&format!("{}/alice29.txt", CANTRBRY_DIR));
    }

    #[test]
    fn asyoulik() {
        do_work(&format!("{}/asyoulik.txt", CANTRBRY_DIR));
    }

    #[test]
    fn cp() {
        do_work(&format!("{}/cp.html", CANTRBRY_DIR));
    }

    #[test]
    fn fields() {
        do_work(&format!("{}/fields.c", CANTRBRY_DIR));
    }

    #[test]
    fn grammar() {
        do_work(&format!("{}/grammar.lsp", CANTRBRY_DIR));
    }

    #[test]
    fn kennedy() {
        do_work(&format!("{}/kennedy.xls", CANTRBRY_DIR));
    }

    #[test]
    fn lcet10() {
        do_work(&format!("{}/lcet10.txt", CANTRBRY_DIR));
    }

    #[test]
    fn plrabn12() {
        do_work(&format!("{}/plrabn12.txt", CANTRBRY_DIR));
    }

    #[test]
    fn ptt5() {
        do_work(&format!("{}/ptt5", CANTRBRY_DIR));
    }

    #[test]
    fn sum() {
        do_work(&format!("{}/sum", CANTRBRY_DIR));
    }

    #[test]
    fn xargs() {
        do_work(&format!("{}/xargs.1", CANTRBRY_DIR));
    }
}
