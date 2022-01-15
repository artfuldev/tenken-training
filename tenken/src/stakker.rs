use actix::Addr;
use stakker::*;
use std::{time::{Instant, Duration}};
use crate::writer::{Writer, Log};

// An actor is represented as a struct which holds the actor state
struct Light {
    start: Instant,
    on: bool,
    writer: Addr<Writer>,
}

impl Light {
    // This is a "Prep" method which is used to create a Self value
    // for the actor.  `cx` is the actor context and gives access to
    // Stakker `Core`.  (`CX![]` expands to `&mut Cx<'_, Self>`.)
    // A "Prep" method doesn't have to return a Self value right away.
    // For example it might asynchronously attempt a connection to a
    // remote server first before arranging a call to another "Prep"
    // function which returns the Self value.  Once a value is returned,
    // the actor is "Ready" and any queued-up operations on the actor
    // will be executed.
    pub fn init(cx: CX![], writer: Addr<Writer>) -> Option<Self> {
        // Use cx.now() instead of Instant::now() to allow execution
        // in virtual time if supported by the environment.
        let start = cx.now();
        Some(Self { writer, start, on: false })
    }

    // Methods that may be called once the actor is "Ready" have a
    // `&mut self` or `&self` first argument.
    pub fn set(&mut self, cx: CX![], on: bool) {
        self.on = on;
        let time = cx.now() - self.start;
        self.writer.do_send(Log::Debug(format!(
            "{:04}.{:03} Light on: {}",
            time.as_secs(),
            time.subsec_millis(),
            on
        )));
    }

    // A `Fwd` or `Ret` allows passing data to arbitrary destinations,
    // like an async callback.  Here we use it to return a value.
    pub fn query(&self, cx: CX![], ret: Ret<bool>) {
        ret!([ret], self.on);
    }
}

// This is another actor that holds a reference to a Light actor.
struct Flasher {
    writer: Addr<Writer>,
    light: Actor<Light>,
    interval: Duration,
    count: usize,
}

impl Flasher {
    pub fn init(cx: CX![], writer: Addr<Writer>, light: Actor<Light>, interval: Duration, count: usize) -> Option<Self> {
        // Defer first switch to the queue
        call!([cx], switch(true));
        Some(Self {
            writer,
            light,
            interval,
            count,
        })
    }

    pub fn switch(&mut self, cx: CX![], on: bool) {
        // Change the light state
        call!([self.light], set(on));

        self.count -= 1;
        if self.count != 0 {
            // Call switch again after a delay
            after!(self.interval, [cx], switch(!on));
        } else {
            // Terminate the actor successfully, causing StopCause handler to run
            cx.stop();
        }

        // Query the light state, receiving the response in the method
        // `recv_state`, which has both fixed and forwarded arguments.
        let ret = ret_some_to!([cx], recv_state(self.count) as (bool));
        call!([self.light], query(ret));
    }

    fn recv_state(&self, _: CX![], count: usize, state: bool) {
        self.writer.do_send(Log::Debug(format!("  (at count {} received: {})", count, state)));
    }
}

pub(crate) fn main(writer: Addr<Writer>) {
    let mut stakker0 = Stakker::new(Instant::now());
    let stakker = &mut stakker0;

    // Create and initialise the Light and Flasher actors.  The
    // Flasher actor is given a reference to the Light.  Use a
    // StopCause handler to shutdown when the Flasher terminates.
    let light = actor!(stakker, Light::init(writer.clone()), ret_nop!());
    let _flasher = actor!(
        stakker,
        Flasher::init(writer.clone(), light.clone(), Duration::from_millis(1000), 600000),
        ret_shutdown!(stakker)
    );

    // Since we're not in virtual time, we use `Instant::now()` in
    // this loop, which is then passed on to all the actors as
    // `cx.now()`.  (If you want to run time faster or slower you
    // could use another source of time.)  So all calls in a batch of
    // processing get the same `cx.now()` value.  Also note that
    // `Instant::now()` uses a Mutex on some platforms so it saves
    // cycles to call it less often.
    stakker.run(Instant::now(), false);
    while stakker.not_shutdown() {
        // Wait for next timer to expire.  Here there's no I/O polling
        // required to wait for external events, so just `sleep`
        let wait_duration = stakker.next_wait_max(Instant::now(), Duration::from_millis(0), false);
        std::thread::sleep(wait_duration);

        // Run queue and timers
        stakker.run(Instant::now(), false);
    }
}
