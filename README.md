# Ray Tracing in One Weekend
This repository contains my rewrite of the code in the [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) book by Peter Shirley. For now there are implementations in Rust and C++.

# Building and Running
## C++
```bash
cd C++
./build.sh
./output > image.ppm
```

## Rust
```bash
cd Rust
cargo build
cargo run --release > image.ppm
```

# Final Scene
![final scene Rust](/Rust/images/image21.png)
