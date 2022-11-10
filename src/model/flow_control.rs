use crate::constant::size::LOCAL_WINDOW_SIZE;

use crate::constant::size;

pub(crate) struct FlowControl {
    /// 本地窗口大小
    local_window: u32,
    /// 远程窗口大小
    remote_window: u32,
}

impl FlowControl {
    pub fn new(remote: u32) -> Self {
        FlowControl {
            local_window: LOCAL_WINDOW_SIZE,
            remote_window: remote,
        }
    }

    pub fn tune_remote(&mut self, buf: &mut Vec<u8>) {
        let recv_len = buf.len() as u32;

        if self.local_window >= recv_len {
            self.local_window -= recv_len;
        } else {
            let drop_len = recv_len - self.local_window;
            log::debug!("Recv more than expected, drop len {}", drop_len);
            buf.truncate(self.local_window as usize);
            self.local_window = 0;
        }
    }

    pub fn tune_local(&mut self, mut buf: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
        let want_send = buf.len();

        let can_send = {
            let mut can_send = want_send;

            if can_send > self.remote_window as usize {
                can_send = self.remote_window as usize
            }

            if can_send > size::BUF_SIZE {
                can_send = size::BUF_SIZE
            }
            can_send
        };

        let remain = buf.split_off(can_send);
        (buf, remain)
    }

    pub fn add_remote(&mut self, size: u32) {
        self.remote_window += size
    }

    pub fn add_local(&mut self, size: u32) {
        self.local_window += size
    }

    pub fn can_send(&self) -> bool {
        self.remote_window > 0
    }
}
