use serde::{Serialize, Deserialize};
#[derive(Serialize,Deserialize,Debug)]
enum FieldType{
    String,
    Integer,
    Float,
    Boolean,
    Enum(Vec<String>),
    Reference(String),
    List(Box<FieldType>),
}

enum FieldValue{
    
}

struct FieldDefinition{
    id:String,
    name: String,
    field_type: FieldType,
    required: bool,
    unique: bool,
}

struct ElementType{
    id: String,
    name: String,
    fields: Vec<FieldDefinition>
    
}
fn main() {

}
