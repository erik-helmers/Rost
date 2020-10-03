

global_asm!(include_str!("context_switch.s"));


extern "sysv64" {
    pub fn context_switch(cur_rsp: *mut u64, next_rsp: *mut u64);
}
