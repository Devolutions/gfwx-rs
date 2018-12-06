#[macro_use]
extern crate criterion;
extern crate gfwx;

use std::io;

use criterion::{black_box, Criterion};
use gfwx::bits::{BitsIOReader, BitsIOWriter, BitsReader, BitsWriter};

struct BenchWriter {
    buffer: [u8; 4],
}

impl BenchWriter {
    fn new() -> BenchWriter {
        BenchWriter { buffer: [0; 4] }
    }
}

impl io::Write for BenchWriter {
    fn write(&mut self, value: &[u8]) -> io::Result<usize> {
        for (b, v) in self.buffer.iter_mut().zip(value.iter()) {
            *b = *v;
        }
        Ok(self.buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct BenchReader {
    value: u8,
}

impl BenchReader {
    fn new() -> BenchReader {
        BenchReader { value: 42 }
    }
}

impl io::Read for BenchReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        for v in buffer.iter_mut() {
            *v = self.value;
        }

        Ok(buffer.len())
    }
}

fn bits_writer_benchmark(c: &mut Criterion) {
    c.bench_function("bits_writer", |b| {
        let mut buff = BenchWriter::new();
        let mut stream = BitsIOWriter::new(&mut buff);
        b.iter(|| {
            for _ in 0..12 {
                black_box(stream.put_bits(15, 4).unwrap());
            }
            stream.flush_write_word().unwrap();
        })
    });
}

fn bits_reader_benchmark(c: &mut Criterion) {
    c.bench_function("bits_reader", |b| {
        let mut source = BenchReader::new();
        let mut stream = BitsIOReader::new(&mut source);
        b.iter(|| {
            for _ in 0..15 {
                black_box(stream.get_bits(4).unwrap());
            }
            stream.flush_read_word();
        })
    });
}

fn bits_zeros_benchmark(c: &mut Criterion) {
    c.bench_function("bits_zeros", |b| {
        let mut source = BenchReader::new();
        let mut stream = BitsIOReader::new(&mut source);
        b.iter(|| {
            for _ in 0..15 {
                black_box(stream.get_zeros(4).unwrap());
            }
            stream.flush_read_word();
        })
    });
}

criterion_group!(
    benches,
    bits_writer_benchmark,
    bits_reader_benchmark,
    bits_zeros_benchmark
);

criterion_main!(benches);
