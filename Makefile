TASKSET = taskset -c 2

BENCH_STR_1 = --bench=bench-teams
BENCH_STR_2 = --bench=bench-big-table

TARGET_GNU = --target=x86_64-unknown-linux-gnu
TARGET_MUSL = --target=x86_64-unknown-linux-musl
TARGET_ARM64_MUSL = --target=aarch64-unknown-linux-musl

ENV = env CARGO_PROFILE_RELEASE_LTO=fat


all:

bench-all: bench-gnu bench-musl

bench-build-all: bench-build-gnu bench-build-musl


bench-gnu: bench.1-gnu bench.2-gnu

bench-musl: bench.1-musl bench.2-musl

bench-arm64-musl: bench.1-arm64-musl bench.2-arm64-musl

bench-build-gnu:
	$(ENV) cargo bench --no-run $(TARGET_GNU)

bench-build-musl:
	$(ENV) cargo bench --no-run $(TARGET_MUSL)

bench-build-arm64-musl:
	$(ENV) cargo bench --no-run $(TARGET_ARM64_MUSL)

bench-clean:
	@rm -fr target/criterion

clean:
	@cargo clean
	@rm -f z.*

report:
	cargo xtask shape_benchmark_results


bench.1-gnu:
	@rm -f z.bench.1.log
	$(ENV) cargo bench --no-run $(TARGET_GNU)
	$(ENV) $(TASKSET) cargo bench $(BENCH_STR_1) $(TARGET_GNU) -- -n | tee -a z.bench.1.log

bench.2-gnu:
	@rm -f z.bench.2.log
	$(ENV) cargo bench --no-run $(TARGET_GNU)
	$(ENV) $(TASKSET) cargo bench $(BENCH_STR_2) $(TARGET_GNU) -- -n | tee -a z.bench.2.log

bench.1-musl:
	@rm -f z.musl.bench.1.log
	$(ENV) cargo bench --no-run $(TARGET_MUSL)
	$(ENV) $(TASKSET) cargo bench $(BENCH_STR_1) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.1.log

bench.2-musl:
	@rm -f z.musl.bench.2.log
	$(ENV) cargo bench --no-run $(TARGET_MUSL)
	$(ENV) $(TASKSET) cargo bench $(BENCH_STR_2) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.2.log

bench.1-arm64-musl:
	@rm -f z.arm64.musl.bench.1.log
	$(ENV) cargo bench --no-run $(TARGET_ARM64_MUSL)
	$(ENV) $(TASKSET) cargo bench $(BENCH_STR_1) $(TARGET_ARM64_MUSL) -- -n | tee -a z.arm64.musl.bench.1.log

bench.2-arm64-musl:
	@rm -f z.arm64.musl.bench.2.log
	$(ENV) cargo bench --no-run $(TARGET_ARM64_MUSL)
	$(ENV) $(TASKSET) cargo bench $(BENCH_STR_2) $(TARGET_ARM64_MUSL) -- -n | tee -a z.arm64.musl.bench.2.log
