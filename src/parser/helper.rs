pub fn get_string_by_offset(raw: &[u8], offset: usize) -> String {
    let mut end = offset;
    while raw[end] != 0 {
        end += 1;
    }

    String::from_utf8_lossy(&raw[offset..end]).to_string()
}

#[test]
fn get_string_by_offset_ok() {
    let bytes = b".text\0";
    let name = get_string_by_offset(bytes, 0);
    assert_eq!(name, ".text");
}
