use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::sync::mpsc::channel;

fn main() {
    let pty_system = NativePtySystem::default();

    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .unwrap();

    let mut cmd = if cfg!(windows) {
        let mut cmd = CommandBuilder::new(r"C:\hostedtoolcache\windows\deno\1.36.3\x64\deno.exe");
        // cmd.args(["-Command", "deno"]);
        cmd
    } else {
        CommandBuilder::new("deno")
    };
    cmd.env("NO_COLOR", "1");
    let _child = pair.slave.spawn_command(cmd).unwrap();

    // Release any handles owned by the slave: we don't need it now
    // that we've spawned the child.
    drop(pair.slave);

    // Read the output in another thread.
    // This is important because it is easy to encounter a situation
    // where read/write buffers fill and block either your process
    // or the spawned process.
    let (tx, rx) = channel();
    let mut reader = pair.master.try_clone_reader().unwrap();
    std::thread::spawn(move || {
        loop {
            // Consume the output from the child
            let mut s = [0; 512];
            let n = reader.read(&mut s).unwrap();
            tx.send(String::from_utf8_lossy(&s[..n]).to_string())
                .unwrap();
        }
    });

    {
        // Obtain the writer.
        // When the writer is dropped, EOF will be sent to
        // the program that was spawned.
        // It is important to take the writer even if you don't
        // send anything to its stdin so that EOF can be
        // generated, otherwise you risk deadlocking yourself.
        let mut writer = pair.master.take_writer().unwrap();

        if cfg!(target_os = "macos") {
            // macOS quirk: the child and reader must be started and
            // allowed a brief grace period to run before we allow
            // the writer to drop. Otherwise, the data we send to
            // the kernel to trigger EOF is interleaved with the
            // data read by the reader! WTF!?
            // This appears to be a race condition for very short
            // lived processes on macOS.
            // I'd love to find a more deterministic solution to
            // this than sleeping.
            std::thread::sleep(std::time::Duration::from_millis(20));
        }

        // This example doesn't need to write anything, but if you
        // want to send data to the child, you'd set `to_write` to
        // that data and do it like this:
        let to_write = "5+4\n";
        if !to_write.is_empty() {
            // To avoid deadlock, wrt. reading and waiting, we send
            // data to the stdin of the child in a different thread.
            std::thread::spawn(move || {
                writer.write_all(to_write.as_bytes()).unwrap();
                dbg!(4);
            });
        }
    }

    // Take care to drop the master after our processes are
    // done, as some platforms get unhappy if it is dropped
    // sooner than that.
    drop(pair.master);

    // Now wait for the output to be read by our reader thread
    for _ in 0..5 {
        dbg!(rx.recv().unwrap());
        std::thread::sleep_ms(100);
    }
}
