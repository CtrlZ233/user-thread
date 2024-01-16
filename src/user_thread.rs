use core::ptr;
use core::arch::asm;

const DEFAULT_STACK_SIZE: usize = 1024 * 4;
const MAX_THREADS: usize = 100;
static mut RUNTIME: usize = 0;

pub struct Runtime {
    threads: Vec<Thread>,
    current: usize,
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Available,
    Running,
    Ready,
}

struct Thread {
    id: usize,
    stack: Vec<u8>,
    ctx: ThreadContext,
    state: State,
}

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
    rdi: u64,
}

impl Thread {
    fn new(id: usize) -> Self {
        Thread {
            id,
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Available,
        }
    }
}

impl Runtime {
    pub fn new() -> Self {
        // 这将是我们的基本线程, 会被初始化为
        // Running 状态
        let base_thread = Thread {
            id: 0,
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Running,
        };

        let mut threads = vec![base_thread];
        let mut available_threads: Vec<Thread> = (1..MAX_THREADS).map(|i| Thread::new(i)).collect();
        threads.append(&mut available_threads);

        Runtime {
            threads,
            current: 0,
        }
    }

    pub fn init(&self) {
        unsafe {
            let r_ptr: *const Runtime = self;
            RUNTIME = r_ptr as usize;
        }
    }

    pub fn run(&mut self) -> ! {
        while self.t_yield() {}
        std::process::exit(0);
    }

    fn t_return(&mut self) {
        if self.current != 0 {
            self.threads[self.current].state = State::Available;
            self.t_yield();
        }
    }

    fn t_yield(&mut self) -> bool {
        let mut pos = self.current;
        while self.threads[pos].state != State::Ready {
            pos += 1;
            if pos == self.threads.len() {
                pos = 0;
            }
            if pos == self.current {
                return false;
            }
        }

        if self.threads[self.current].state != State::Available {
            self.threads[self.current].state = State::Ready;
        }

        self.threads[pos].state = State::Running;
        let old_pos = self.current;
        self.current = pos;

        unsafe {
            switch(&mut self.threads[old_pos].ctx, &self.threads[pos].ctx);
        }

        self.threads.len() > 0
    }

    pub fn spawn(&mut self, f: fn(u64), arg: u64) {
        let available = self
            .threads
            .iter_mut()
            .find(|t| t.state == State::Available)
            .expect("no available thread.");

        let size = available.stack.len();
        let s_ptr = available.stack.as_mut_ptr();

        unsafe {
            ptr::write(s_ptr.offset((size - 24) as isize) as *mut u64, guard as u64);
            ptr::write(s_ptr.offset((size - 32) as isize) as *mut u64, f as u64);
            available.ctx.rsp = s_ptr.offset((size - 32) as isize) as u64;
            available.ctx.rdi = arg;
        }
        available.state = State::Ready;
    }
}

fn guard() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_return();
    };
}

pub fn yield_thread(rt: &mut Runtime) {
    // unsafe {
    //     let rt_ptr = RUNTIME as *mut Runtime;
    //     (*rt_ptr).t_yield();
    // };
    rt.t_yield();
}

#[inline(never)]
unsafe fn switch(old: *mut ThreadContext, new: *const ThreadContext) {
    asm!(
        "mov    [{0}], rsp",
        "mov    [{0} + 0x08], r15",
        "mov    [{0} + 0x10], r14",
        "mov    [{0} + 0x18], r13",
        "mov    [{0} + 0x20], r12",
        "mov    [{0} + 0x28], rbx",
        "mov    [{0} + 0x30], rbp",
        "mov    [{0} + 0x38], rdi",


        "mov    rsp, [{1}]",
        "mov    r15, [{1} + 0x08]",
        "mov    r14, [{1} + 0x10]",
        "mov    r13, [{1} + 0x18]",
        "mov    r12, [{1} + 0x20]",
        "mov    rbx, [{1} + 0x28]",
        "mov    rbp, [{1} + 0x30]",
        "mov    rdi, [{1} + 0x38]",
        "ret",
        in(reg) old as usize,
        in(reg) new as usize,
        options(nomem, nostack, preserves_flags)
    );
}
