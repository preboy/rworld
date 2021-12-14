use std::collections::hash_map::HashMap;

use crate::network::TCPSession;

pub struct NetMgr {
    id_seq: i32,
    sessions: HashMap<i32, TCPSession>,
}

impl NetMgr {
    pub fn new() -> Self {
        Self {
            id_seq: 0,
            sessions: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        for s in self.sessions.values_mut() {
            s.update();
        }
    }

    // 新连接
    pub fn new_connection(&mut self, mut session: TCPSession) {
        self.id_seq += 1;
        session.set_id(self.id_seq);
        self.sessions.insert(self.id_seq, session);
    }

    pub fn stop(&mut self) {
        todo!();
    }
}
