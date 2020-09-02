use std::env;
use std::time::Instant;

#[macro_use]
extern crate microprofile;

trait Test {
    fn fibonacci(&self, n: u32) -> u32;
    fn run(&self, n: u32);
}

struct Optick {}
impl Test for Optick {
    #[optick_attr::profile]
    fn fibonacci(&self, n: u32) -> u32 {
        match n {
            0 => 1,
            1 => 1,
            _ => self.fibonacci(n - 1) + self.fibonacci(n - 2),
        }
    }

    fn run(&self, n: u32) {
        optick::start_capture();

        let mut time = Instant::now();
        self.fibonacci(n);
        let run_duration = time.elapsed();

        time = Instant::now();
        optick::stop_capture("optick_capture");
        let save_duration = time.elapsed();

        println!("Optick time: {:.2?}", run_duration);
        println!("Optick save: {:.2?}", save_duration);
    }

    // #[optick_attr::capture("optick_capture_mt.opt")]
    // fn run_mt(&self, n: u32) {
    //     let mut handles = Vec::new();
    //     for _ in 0..num_cpus::get()/2-1 {
    //         handles.push(thread::spawn(move|| {
    //             optick::register_thread("Worker");
    //             Optick{}.fibonacci(n);
    //         }));
    //     }
    //     Optick{}.fibonacci(n);
    //     for handle in handles {
    //         handle.join().unwrap();
    //     }
    // }
}

struct Superluminal {}
impl Test for Superluminal {
    fn fibonacci(&self, n: u32) -> u32 {
        superluminal_perf::begin_event("fibonacci");
        let res = match n {
            0 => 1,
            1 => 1,
            _ => self.fibonacci(n - 1) + self.fibonacci(n - 2),
        };
        superluminal_perf::end_event();
        return res;
    }

    fn run(&self, n: u32) {
        let start = Instant::now();
        self.fibonacci(n);
        println!("Superluminal time: {:.2?}", start.elapsed());
    }
}

struct Microprofile {}
impl Test for Microprofile {
    fn fibonacci(&self, n: u32) -> u32 {
        microprofile::scope!("foo", "fibonacci");
        match n {
            0 => 1,
            1 => 1,
            _ => self.fibonacci(n - 1) + self.fibonacci(n - 2),
        }
    }

    fn run(&self, n: u32) {
        microprofile::init();
        microprofile::set_enable_all_groups(true);
        microprofile::start_auto_flip(20);

        let mut time = Instant::now();
        self.fibonacci(n);
        let run_duration = time.elapsed();

        microprofile::stop_auto_flip();
        time = Instant::now();
        microprofile::dump_file_immediately("microprofile.html", "");
        let save_duration = time.elapsed();

        println!("Microprofile time: {:.2?}", run_duration);
        println!("Microprofile save: {:.2?}", save_duration);
        microprofile::shutdown();
    }
}


struct Tracy {}
impl Test for Tracy {
    fn fibonacci(&self, n: u32) -> u32 {
        match n {
            0 => 1,
            1 => 1,
            _ => self.fibonacci(n - 1) + self.fibonacci(n - 2),
        }
    }

    fn run(&self, n: u32) {
        loop {
            tracy_rs::tracy_begin_frame!("MainThread");
            let time = Instant::now();
            self.fibonacci(n);
            let run_duration = time.elapsed();
            tracy_rs::tracy_end_frame!("MainThread");
            println!("Tracy time: {:.2?}", run_duration);
        }
    }
}

struct Puffin {}
impl Test for Puffin {
    fn fibonacci(&self, n: u32) -> u32 {
        puffin::profile_function!();
        let res = match n {
            0 => 1,
            1 => 1,
            _ => self.fibonacci(n - 1) + self.fibonacci(n - 2),
        };
        return res;
    }

    fn run(&self, n: u32) {
        puffin::set_scopes_on(true);
        let start = Instant::now();
        self.fibonacci(n);
        println!("Puffin time: {:.2?}", start.elapsed());
    }
}

/// Usage: profiler-bench-rs optick 30
pub fn main() {
    let args: Vec<String> = env::args().collect();

    let mut profiler = "optick";
    if args.len() > 1 {
        profiler = &args[1];
    }
        
    let mut number: u32 = 23;
    if args.len() > 2 {
        number = (&args[2]).parse().unwrap();
    }

    println!("Running benchmark: {} (N={})", profiler, number);

    match &profiler[..] {
        "optick" => Optick {}.run(number),
        "superluminal" => Superluminal {}.run(number),
        "microprofile" => Microprofile {}.run(number),
        "tracy" => Tracy {}.run(number),
        "puffin" => Puffin {}.run(number),
        _ => println!("Can't find a test for {}!", profiler),
    }
}