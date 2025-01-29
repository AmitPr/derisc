use criterion::{criterion_group, criterion_main, Criterion};
use riscv_kernel_linux::MockLinux;
use riscv_vm::machine::Machine;

fn primes_setup() -> Machine<MockLinux> {
    // TODO: Hardcoded path for now.
    const PRIMES_ELF_FILE: &str = "/Users/amit/Documents/projects/derisc/riscv/guest_std/target/riscv32imac-unknown-linux-musl/release/guest_std";
    let elf = std::fs::read(PRIMES_ELF_FILE).expect("Failed to read ELF file");

    let mut machine = Machine::new(MockLinux::new(false));
    machine
        .kernel
        .load_static_elf(&mut machine.hart, &mut machine.mem, &elf, &[], &[]);

    machine
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("primes_bench", |b| {
        b.iter_batched(
            primes_setup,
            |mut machine| {
                machine.run().expect("Failed to run");
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
