#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[bench]
    fn bench_parse_elf_header(b: &mut Bencher) {
        let raw = include_bytes!("../src/parser/fixtures/sub.o");
        b.iter(|| {
            let _ = yui::parser::header::parse(raw).unwrap();
        });
    }
}
