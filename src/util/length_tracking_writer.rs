// length_tracking_writer.rs

use std::io::{self, Write};

/// `LengthTrackingWriter` 结构体用于包装一个实现了 `Write` trait 的对象，同时跟踪写入的数据长度。
pub struct LengthTrackingWriter<W: Write> {
    inner: W,
    length: usize,
    /// 存储写入的数据
    buffer: Vec<u8>, // 使 buffer 成为公有以便在 txid 函数中访问
}

impl<W: Write> LengthTrackingWriter<W> {
    /// 创建一个新的 `LengthTrackingWriter` 实例。
    ///
    /// # 参数
    ///
    /// * `inner` - 被包装的写入对象。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `LengthTrackingWriter` 实例。
    pub fn new(inner: W) -> Self {
        LengthTrackingWriter {
            inner,
            length: 0,
            buffer: Vec::new(),
        }
    }

    /// 获取已经写入的数据长度。
    ///
    /// # 返回值
    ///
    /// 返回已经写入的数据长度。
    pub fn length(&self) -> usize {
        self.length
    }

    /// 获取已经写入的数据内容。
    ///
    /// # 返回值
    ///
    /// 返回已经写入的数据内容。
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}

impl<W: Write> Write for LengthTrackingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let bytes_written = self.inner.write(buf)?;
        self.length += bytes_written;
        self.buffer.extend_from_slice(&buf[..bytes_written]);
        Ok(bytes_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
