pub struct Key(Vec<u8>);
pub struct Value(Vec<u8>);

pub struct Item {
    pub key: Key,
    pub value: Value,
}

pub struct DB {}
