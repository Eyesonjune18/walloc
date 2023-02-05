use walloc::mem::Memory;

fn main() {
    let _big_mem = Memory::from_terabytes(999999999).unwrap();

    loop {}
}
