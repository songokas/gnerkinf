use std::io::{stdout, StdoutLock, Write};

pub fn create_stdout_writer() -> impl Write {
    SeperatorLogger {
        writer: stdout().lock(),
    }
}

struct SeperatorLogger {
    writer: StdoutLock<'static>,
}

impl Drop for SeperatorLogger {
    fn drop(&mut self) {
        writeln!(self.writer, "=========================").expect("failed to write separator");
        self.writer.flush().expect("failed to flush")
    }
}

impl Write for SeperatorLogger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
