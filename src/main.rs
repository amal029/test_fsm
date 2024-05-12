mod threads;
// XXX: First declare the thread struct that carries data

trait State {}

#[derive(Debug)]
struct Data {
    a: i32,
}

#[derive(Debug)]
struct Thread<S: State> {
    d: Data,  //the data that we will work on
    _curr: S, //the current state we are in
}

#[derive(Debug)]
struct S0 {}
#[derive(Debug)]
struct S1 {}
#[derive(Debug)]
struct S2 {}
#[derive(Debug)]
struct S3 {}

// XXX: Implement the State trair for each State
impl State for S0 {}
impl State for S1 {}
impl State for S2 {}
impl State for S3 {}

// XXX: The enumeration for the thread state
#[derive(Debug)]
enum ThreadState {
    S0(Thread<S0>),
    S1(Thread<S1>),
    S2(Thread<S2>),
    S3(Thread<S3>),
}

impl ThreadState {
    fn tick(self) -> ThreadState {
        match self {
            ThreadState::S0(x) => x.tick(),
            ThreadState::S1(x) => x.tick(),
            ThreadState::S2(x) => x.tick(),
            ThreadState::S3(x) => x.tick(),
        }
    }
}

// XXX: S0 -> S1
impl Thread<S0> {
    fn new() -> ThreadState {
        ThreadState::S0(Thread {
            d: Data { a: 0 },
            _curr: S0 {},
        })
    }
    // XXX: This is the transition from S0 --> S1
    fn tick(self) -> ThreadState {
        let a = self.d.a;
        ThreadState::S1(Thread::<S1> {
            d: Data { a: a + 1 },
            _curr: S1 {},
        })
    }
}

// XXX: S1 -> S2 || S1 -> S3
impl Thread<S1> {
    fn tick(self) -> ThreadState {
        if self.d.a == 1 {
            ThreadState::S2(Thread {
                d: self.d,
                _curr: S2 {},
            })
        } else if self.d.a != 1 {
            ThreadState::S3(Thread {
                d: self.d,
                _curr: S3 {},
            })
        } else {
            panic!("Wrong branch")
        }
    }
}

// XXX: S2 -> S1 forever
impl Thread<S2> {
    fn tick(self) -> ThreadState {
        ThreadState::S1(Thread {
            d: Data { a: self.d.a + 1 },
            _curr: S1 {},
        })
    }
}

// XXX: from S3 -> S1
impl Thread<S3> {
    fn tick(self) -> ThreadState {
        ThreadState::S1(Thread::<S1> {
            d: Data { a: self.d.a - 1 },
            _curr: S1 {},
        })
    }
}

fn main() {
    let mut t = Thread::new();
    println!("{:?}", t);
    // XXX: Do 10 iterations of the FSM
    for _ in 0..10 {
        // XXX: Make the tick transition
        t = t.tick();
        println!("{:?}", t);
    }

    // XXX: The idea is that each thread runs each tick. The main thread
    // then runs its ticks, syncronizing and making more ticks in the
    // threads if necessary like sync charts.

    // XXX: The data is sent as clone in threads if necessary. Combined
    // once the threads do their ticks. Finally, cloned data is killed
    // once the threads are done.
    threads::_thread_processing();
}
