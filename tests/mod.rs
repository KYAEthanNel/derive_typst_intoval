use derive_typst_intoval::{IntoDict, IntoValue};
use typst::foundations::{dict, IntoValue, Value};

#[derive(IntoValue, IntoDict, Clone)]
struct MyStruct {
  field1: &'static str,
}

#[test]
fn basic() {
  let m = MyStruct { field1: "xyz" };

  let v = Value::Dict(dict!(
      "field1" => "xyz".into_value(),
  ));

  assert_eq!(m.clone().into_value(), v);
  assert_eq!(Value::Dict(m.into_dict()), v);
}

#[derive(IntoValue, IntoDict, Clone)]
#[rename("AsLowerCamelCase")]
struct MyStruct2 {
  field_name: &'static str,
}

#[test]
fn rename_global() {
  let m = MyStruct2 { field_name: "xyz" };

  let v = Value::Dict(dict!(
      "fieldName" => "xyz".into_value(),
  ));

  assert_eq!(m.clone().into_value(), v);
  assert_eq!(Value::Dict(m.into_dict()), v);
}

#[derive(IntoValue, IntoDict, Clone)]
struct Rename {
  #[rename("customfieldname")]
  field1: &'static str,
}

#[test]
fn renamimg() {
  let m = Rename { field1: "xyz" };

  let v = Value::Dict(dict!(
      "customfieldname" => "xyz".into_value(),
  ));

  assert_eq!(m.clone().into_value(), v);
  assert_eq!(Value::Dict(m.into_dict()), v);
}

#[derive(IntoValue, IntoDict, Clone)]
struct Nested {
  field3: MyStruct,
}

#[test]
fn nesting() {
  let mystruct = MyStruct { field1: "xyx" };
  let m = Nested {
    field3: mystruct.clone(),
  };

  let v = Value::Dict(dict!(
      "field3" => mystruct.into_value(),
  ));

  assert_eq!(m.clone().into_value(), v);
  assert_eq!(Value::Dict(m.into_dict()), v);
}

#[derive(IntoDict, IntoValue, Clone)]
struct CustomMethod {
  #[serialize_with(custom_func)]
  field4: f32,
}

fn custom_func(input: f32) -> Value {
  Value::Float(input as f64)
}

#[test]
fn custom_serializer() {
  let m = CustomMethod { field4: 32.00 };

  let v = Value::Dict(dict!(
      "field4" => Value::Float(32.00),
  ));

  assert_eq!(m.clone().into_value(), v);
  assert_eq!(Value::Dict(m.into_dict()), v);
}
