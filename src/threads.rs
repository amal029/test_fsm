use std::{
    sync::mpsc::{channel, Sender},
    thread::spawn,
};

pub fn __ts(id: u8, a: SomeData, _tx: Sender<(TState, SomeData, Sender<(bool, SomeData)>)>) {
    println!("Hello from thread, {id}, {:?}", a);
    let mut u = a;
    let (tx, rx) = channel::<(bool, SomeData)>();
    // XXX: Tick start
    loop {
        u._a += 1;
        // let uu = u.clone();
        let txx = tx.clone();
        let _ = match id {
            0 => _tx.send((TState::S0, u, txx)),
            1 => _tx.send((TState::S1, u, txx)),
            2 => _tx.send((TState::S2, u, txx)),
            _ => _tx.send((TState::D, u, txx)),
        };
        // XXX: Tick end
        let (v, uu) = rx.recv().unwrap();
        if v {
            u = uu;
            continue;
        } else {
            break;
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct SomeData {
    _a: usize,
}

impl Default for SomeData {
    fn default() -> Self {
        SomeData { _a: 0 }
    }
}

#[derive(Debug, Clone)]
pub enum TState {
    S0,
    S1,
    S2,
    D,
}

impl std::ops::Add for SomeData {
    type Output = Self;
    fn add(self, other: SomeData) -> Self {
        SomeData {
            _a: self._a + other._a,
        }
    }
}

pub fn _thread_processing() {
    let nthreads: u8 = 3;
    let mut hs = Vec::with_capacity(nthreads as usize);
    let _aa = SomeData::default();
    let (tx, rx) = channel::<(TState, SomeData, Sender<(bool, SomeData)>)>();
    for i in 0..nthreads {
        let m = _aa.clone(); //send a clone/deep copy of the data to each thread
        let _tx = tx.clone(); //the updated data will be sent back to
                              // main via this channel
        let h = spawn(move || __ts(i, m, _tx));
        hs.push(h);
    }
    let mut rcs: Vec<(TState, SomeData, Sender<(bool, SomeData)>)> =
        Vec::with_capacity(nthreads as usize);
    for _ in 0..10 {
        rcs.clear();
        // XXX: Receive the message back from the threads
        for _ in 0..nthreads {
            rcs.push(rx.recv().unwrap());
        }
        println!("{:?}", rcs);
        // XXX: Combine all received values using some operator like +
        let dtos = rcs
            .iter()
            .map(|x| x.1)
            .fold(SomeData::default(), |acc, x| acc + x);
        // XXX: Send true on the channel received
        for i in &rcs {
            let _ = i.2.send((true, dtos));
        }
    }
    // XXX: Send the final false to each thread
    for i in &rcs {
        let _ = i.2.send((false, SomeData::default()));
    }
    println!("{:?}", rcs);
    // XXX: Finished and killed all threads
    for h in hs {
        h.join().unwrap();
    }
}
