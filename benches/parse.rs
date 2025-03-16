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

    #[bench]
    fn bench_parse_section_header_table(b: &mut Bencher) {
        let raw = include_bytes!("../src/parser/fixtures/sub.o");
        let (_, ident) = yui::parser::header::parse(raw).unwrap();
        b.iter(|| {
            let _ = yui::parser::section::parse_header_table(
                raw,
                ident.shoff as usize,
                ident.shstrndx as usize,
                ident.shnum as usize,
            )
            .unwrap();
        });
    }
}
