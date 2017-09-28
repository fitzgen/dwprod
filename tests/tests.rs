use std::process::Command;
use std::str;

#[test]
#[cfg(feature = "exe")]
fn dwprod_has_rustc_producer() {
    let dwprod = env!("DWPROD_EXE");

    let mut cmd = Command::new(dwprod);
    cmd.arg(dwprod);
    println!("Running {:?}", cmd);

    let output = cmd
        .output()
        .expect("should run ok");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());

    assert!(
        str::from_utf8(&output.stdout)
            .expect("dwprod should write valid utf8 to stdout")
            .contains("rustc")
    );
}
