use crate::bindings;

pub struct Fd(pub bindings::ioiFileDescriptor_t);

impl core::fmt::Write for Fd {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut already_written = 0;

        while already_written < s.len() {
            already_written +=
                unsafe { bindings::write(self.0, s.as_ptr() as _, s.len() - already_written) }
                    as usize;
        }
        Ok(())
    }
}
