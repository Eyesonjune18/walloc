use walloc::mem::Memory;

fn main() {
    let _big_mem = Memory::from_gigabytes(8);

    loop {}
}
