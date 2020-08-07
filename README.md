Performance Benchmark for Rust Profilers

Test PC:
- CPU: Intel(R) Core(TM) i7-7700HQ CPU @ 2.80GHz (8 CPUs), ~2.8GHz
- RAM: 16 GB
- HDD: Samsung SSD 970 EVO 1TB

# Benchmark
The benchmark consists of three tests:
1. Small: ~92k events (run: `profiler-bench-rs [profiler] 23`)
2. Medium: ~2.7kk events (run: `profiler-bench-rs [profiler] 30`) 
2. Large: ~18.4kk events (run: `profiler-bench-rs [profiler] 34`) 

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
| Name | URL | Version | Screenshot |
| ---- | --- | ------- | ---------- |
| Optick | https://crates.io/crates/optick | optick = "1.3.4" | | 
| Microprofile | https://crates.io/crates/microprofile | microprofile = "0.2.1" | |
| Superluminal | https://crates.io/crates/superluminal-perf | superluminal-perf = "0.1.1" | |

# Performance Overhead
| Runtime Overhead  | Optick | Microprofile | Superluminal |
| ------------- | -------------:| -----:| ------:|
| Small (~92k events)    | 0.005 sec | 0.006 sec | 0.864 sec |
| Medium (~2.7kk events) | 0.153 sec | Failed | 23.940 sec |
| Large (~18.4kk events) | 0.955 sec | Failed | 168.312 sec |
