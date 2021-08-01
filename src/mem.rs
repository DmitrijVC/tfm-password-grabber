use super::*;


pub(crate) unsafe fn find_addr(ptr: uintptr_t, offsets: &[usize]) -> uintptr_t{
    let mut addr: uintptr_t = ptr;

    for offset in offsets {
        if IsBadReadPtr(
            (addr as *const uintptr_t) as *const _,
            std::mem::size_of::<uintptr_t>(),
        ) != 0 {
            return 0;
        };

        addr = *(addr as *mut uintptr_t);
        addr += *offset;
    }

    return addr;
}
