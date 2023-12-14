pub struct Field {
  name: String,
  field_type: FieldType,
  tag: Option<FiledTag>,
}

enum FieldType {
  Str,
  Int64,
  Struct,
  List,
}

enum FiledTag {
  Id,
  Phone,
  Image,
  File,
  LongText,
}
