use std::io::{self, Result, Write, Stdout};
use std::thread;
use std::time::Duration;



pub fn test() -> Result<()> {

    //clear_screen().unwrap();
    let mut stdout: Stdout = io::stdout();

    init_screen_area(&mut stdout)
}


fn clear_screen() -> Result<()> {
    let mut stdout: Stdout = io::stdout();

    thread::sleep(Duration::from_secs(2));

    stdout.write_all(b"\x1B[H").unwrap();
    stdout.write_all(b"\x1B[2J").unwrap();

    stdout.flush().unwrap();


    Ok(())
}


fn init_screen_area(stdout: &mut Stdout) -> Result<()> {

    stdout.write_all(b"\x1B[2J\x1B[H")?;

    const W: usize = 92;
    const H: usize = 22;

    let wst: String = String::from_iter(['='; W].iter());
    let wbuf: &[u8] = wst.as_bytes();

    let hst: String = String::from("|") + &String::from_iter([' '; W-2].iter()) + "|";
    let hbuf: &[u8] = hst.as_bytes();

    stdout.write_all(wbuf)?;
    stdout.write(b"\n")?;
    for _i in 0..H {
        stdout.write_all(hbuf)?;
        stdout.write(b"\n")?;
    }

    stdout.write_all(wbuf)?;
    stdout.flush()
}