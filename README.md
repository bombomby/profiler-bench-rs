# Performance Benchmark for Rust Profilers
A benchmark for comparing differrent performance profilers for Rust.

# Benchmark
The benchmark consists of three tests:
| Test     | Number of Events | Command |
| -------- | ----------------:| ------- |
| Small    | 92 735     | `profiler-bench-rs [profiler] 23` |
| Medium   | 2 692 537  | `profiler-bench-rs [profiler] 30` |
| Large    | 18 454 929 | `profiler-bench-rs [profiler] 34` |

Each test calculates N-th fibonacci number:
```rust
fn fibonacci(&self, n: u32) -> u32 {
    // PROFILER: START EVENT
    match n {
        0 => 1,
        1 => 1,
        _ => self.fibonacci(n - 1) + self.fibonacci(n - 2),
    }
    // PROFILER: STOP EVENT
}
```
# Profilers
| Name | Crate | Version |
| ---- | ----- | ------- |
| Optick | https://crates.io/crates/optick | optick = "1.3.4" |
| Microprofile | https://crates.io/crates/microprofile | microprofile = "0.2.1" |
| Superluminal | https://crates.io/crates/superluminal-perf | superluminal-perf = "0.1.1" |


# Test PC
* CPU: Intel(R) Core(TM) i7-7700HQ CPU @ 2.80GHz (8 CPUs), ~2.8GHz
* RAM: 16 GB
* HDD: Samsung SSD 970 EVO 1TB

# Runtime Overhead
| Runtime Overhead       | Optick    | Microprofile | Superluminal |
| ---------------------- | ---------:| ------------:| ------------:|
| Small (~92k events)    | 0.005 sec | 0.006 sec    | 0.864 sec    |
| Medium (~2.7kk events) | 0.153 sec | Failed       | 23.940 sec   |
| Large (~18.4kk events) | 0.955 sec | Failed       | 168.312 sec  |

# Capture Save Time
| Capture Save Time      | Optick    | Microprofile | Superluminal |
| ---------------------- | ---------:| ------------:| ------------:|
| Small (~92k events)    | 0.6 sec   | 0.6 sec      | 2.3 sec      |
| Medium (~2.7kk events) | 1.3 sec   | Failed       | 13.3 sec     |
| Large (~18.4kk events) | 5.5 sec   | Failed       | 41.4 sec     |

# Capture Open Time
| Capture Open Time      | Optick    | Microprofile | Superluminal |
| ---------------------- | ---------:| ------------:| ------------:|
| Small (~92k events)    | 0.9 sec   | 1.4 sec      | 2.8 sec      |
| Medium (~2.7kk events) | 6.5 sec   | Failed       | 36.6 sec     |
| Large (~18.4kk events) | 33.4 sec  | Failed       | Failed       |

# Capture Size
| Capture Size           | Optick    | Microprofile | Superluminal |
| ---------------------- | ---------:| ------------:| ------------:|
| Small (~92k events)    | 1.1 MB    | 2.8 MB       | 52 MB        |
| Medium (~2.7kk events) | 8.9 MB    | Failed       | 1347 MB      |
| Large (~18.4kk events) | 56.2 MB   | Failed       | 3413 MB      |

