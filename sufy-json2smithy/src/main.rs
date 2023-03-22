fn main() {
    use atelier_core::io::{read_model_from_string, write_model_to_string};
    use atelier_core::model::NamespaceID;
    use atelier_json::JsonReader;
    use atelier_smithy::SmithyWriter;
    use std::io::{Read, Write};
    let mut json = String::new();
    std::fs::File::open("object.json")
        .unwrap()
        .read_to_string(&mut json)
        .unwrap();

    let mut reader = JsonReader::default();
    let model = read_model_from_string(&mut reader, json).unwrap();

    let mut writer = SmithyWriter::new(NamespaceID::new_unchecked("com.sufybkt.object"));
    let smithy_str = write_model_to_string(&mut writer, &model).unwrap();

    std::fs::File::create("object.smithy")
        .unwrap()
        .write_all(smithy_str.as_bytes())
        .unwrap();
}
