use std::io;

fn main() {
    let r = io::stdin();
    let mut reader = r.lock();

    let w = io::stdout();
    let mut writer = w.lock();

    let _ = io::copy(&mut reader, &mut writer);
}
