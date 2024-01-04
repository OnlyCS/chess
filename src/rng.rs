use std::ffi::c_int;

#[link(name = "c")]
extern "C" {
    fn srandom(seed: c_int);
    fn random() -> c_int;
}

pub fn dense_random() -> u64 {
    let n1 = unsafe { random() & 0xffff } as u64;
    let n2 = unsafe { random() & 0xffff } as u64;
    let n3 = unsafe { random() & 0xffff } as u64;
    let n4 = unsafe { random() & 0xffff } as u64;

    return n1 | (n2 << 16) | (n3 << 32) | (n4 << 48);
}

pub fn init() {
    let now = std::time::SystemTime::now();
    let unix = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    let seed = unix.subsec_nanos();

    unsafe {
        srandom(seed as i32);
    }
}
