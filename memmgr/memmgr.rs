use std::ptr;
use std::mem::ManuallyDrop;

// Type aliases equivalent to C typedefs
type Byte = u8;
type Ulong = usize;
type Align = usize;

// Memory header union and struct
#[repr(C)]
union MemHeaderUnion {
    s: ManuallyDrop<MemHeaderStruct>,
    align_dummy: Align,
}

#[repr(C)]
struct MemHeaderStruct {
    next: *mut MemHeaderUnion,
    size: Ulong,
}

type MemHeaderT = MemHeaderUnion;

// Static variables
static mut BASE: MemHeaderT = MemHeaderT {
    s: ManuallyDrop::new(MemHeaderStruct {
        next: ptr::null_mut(),
        size: 0,
    }),
};
static mut FREEP: *mut MemHeaderT = ptr::null_mut();
static mut POOL: [Byte; 8 * 1024] = [0; 8 * 1024];
static mut POOL_FREE_POS: Ulong = 0;

// Memory manager initialization
#[no_mangle]
pub extern "C" fn memmgr_init() {
    unsafe {
        BASE.s = ManuallyDrop::new(MemHeaderStruct {
            next: ptr::null_mut(),
            size: 0,
        });
        FREEP = ptr::null_mut();
        POOL_FREE_POS = 0;
    }
}

// Get memory from pool (helper function)
unsafe fn get_mem_from_pool(mut nquantas: Ulong) -> *mut MemHeaderT {
    let total_req_size: Ulong;
    let h: *mut MemHeaderT;

    if nquantas < 16 {
        nquantas = 16;
    }

    total_req_size = nquantas * std::mem::size_of::<MemHeaderT>();

    if POOL_FREE_POS + total_req_size <= 8 * 1024 {
        h = POOL.as_mut_ptr().add(POOL_FREE_POS) as *mut MemHeaderT;
        (*h).s = ManuallyDrop::new(MemHeaderStruct {
            next: ptr::null_mut(),
            size: nquantas,
        });
        memmgr_free(h.add(1) as *mut _);
        POOL_FREE_POS += total_req_size;
    } else {
        return ptr::null_mut();
    }

    FREEP
}

// Memory allocation
#[no_mangle]
pub extern "C" fn memmgr_alloc(nbytes: Ulong) -> *mut std::ffi::c_void {
    unsafe {
        let mut p: *mut MemHeaderT;
        let mut prevp: *mut MemHeaderT;
        
        let nquantas = (nbytes + std::mem::size_of::<MemHeaderT>() - 1) / 
                       std::mem::size_of::<MemHeaderT>() + 1;

        prevp = FREEP;
        if prevp.is_null() {
            BASE.s = ManuallyDrop::new(MemHeaderStruct {
                next: &mut BASE,
                size: 0,
            });
            FREEP = &mut BASE;
            prevp = &mut BASE;
        }

        p = (*prevp).s.next;
        loop {
            if (*p).s.size >= nquantas {
                if (*p).s.size == nquantas {
                    (&mut *(*prevp).s).next = (*p).s.next;
                } else {
                    (&mut *(*p).s).size -= nquantas;
                    p = p.add((*p).s.size);
                    (*p).s = ManuallyDrop::new(MemHeaderStruct {
                        next: ptr::null_mut(),
                        size: nquantas,
                    });
                }
                FREEP = prevp;
                return p.add(1) as *mut std::ffi::c_void;
            }

            if p == FREEP {
                p = get_mem_from_pool(nquantas);
                if p.is_null() {
                    return ptr::null_mut();
                }
            }

            prevp = p;
            p = (*p).s.next;
        }
    }
}

// Memory deallocation
#[no_mangle]
pub extern "C" fn memmgr_free(ap: *mut std::ffi::c_void) {
    unsafe {
        let block: *mut MemHeaderT = (ap as *mut MemHeaderT).sub(1);
        let mut p: *mut MemHeaderT = FREEP;

        while !((block > p && block < (*p).s.next) || 
                (p >= (*p).s.next && (block > p || block < (*p).s.next))) {
            if p >= (*p).s.next && (block > p || block < (*p).s.next) {
                break;
            }
            p = (*p).s.next;
        }

        if block.add((*block).s.size) == (*p).s.next {
            (&mut *(*block).s).size += (*(*p).s.next).s.size;
            (&mut *(*block).s).next = (*(*p).s.next).s.next;
        } else {
            (&mut *(*block).s).next = (*p).s.next;
        }

        if p.add((*p).s.size) == block {
            (&mut *(*p).s).size += (*block).s.size;
            (&mut *(*p).s).next = (*block).s.next;
        } else {
            (&mut *(*p).s).next = block;
        }

        FREEP = p;
    }
}

// Optional: Add memmgr_print_stats if needed
#[no_mangle]
pub extern "C" fn memmgr_print_stats() {
    // Implementation would depend on what stats you want to track
}