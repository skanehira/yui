use super::ParseResult;

pub fn get_string_by_offset(raw: &[u8], offset: usize) -> ParseResult<String> {
    let mut end = offset;
    while raw[end] != 0 {
        end += 1;
    }

    let name = String::from_utf8_lossy(&raw[offset..end]).to_string();
    Ok((&[], name))
}

#[test]
fn get_string_by_offset_ok() {
    let bytes = b".text\0";
    let (_, name) = get_string_by_offset(bytes, 0).unwrap();
    assert_eq!(name, ".text");
}
