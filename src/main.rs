use walloc::mem::Memory;

fn main() {
    let _big_mem = Memory::from_gigabytes(20).unwrap();

    loop {}
}
