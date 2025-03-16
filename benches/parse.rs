#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use yui::elf;

    #[bench]
    fn bench_parse_elf_header(b: &mut Bencher) {
        let raw = include_bytes!("../src/elf/fixtures/sub.o");
        b.iter(|| {
            let _ = elf::parser::parse_elf(raw);
        });
    }
}
