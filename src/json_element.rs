use std::ops::Index;

use crate::json_elements::json_array::JsonArray;
use crate::json_elements::json_number::JsonNumber;
use crate::json_elements::json_object::JsonObject;
use crate::json_types::JsonType;
use crate::json_types::JsonType::JsonEmpty;
use crate::slice::Slice;

static EMPTY_ELEMENT: JsonElement = JsonElement::empty();

#[derive(Debug)]
pub struct JsonElement<'a> {
    pub json_type: JsonType,
    pub slice: Slice<'a>,
    pub boolean: Option<bool>,
    pub number: Option<JsonNumber>,
    pub object: Option<JsonObject<'a>>,
    pub array: Option<JsonArray<'a>>,
}

impl<'a> JsonElement<'_> {
    pub fn get_type(&self) -> &JsonType {
        return &self.json_type;
    }

    pub fn get_slice(&self) -> &Slice {
        return &self.slice;
    }

    fn from_type_slice(json_type: JsonType, slice: Slice<'a>) -> JsonElement<'a> {
        return JsonElement {
            json_type,
            slice,
            boolean: None,
            number: None,
            object: None,
            array: None,
        };
    }

    pub fn from_null(slice: Slice<'a>) -> JsonElement<'a> {
        return JsonElement::from_type_slice(JsonType::JsonNull, slice);
    }

    pub fn from_boolean(boolean: bool, slice: Slice<'a>) -> JsonElement<'a> {
        return JsonElement {
            json_type: JsonType::JsonBoolean,
            slice,
            boolean: Some(boolean),
            number: None,
            object: None,
            array: None,
        };
    }

    pub fn from_string(slice: Slice<'a>) -> JsonElement<'a> {
        return JsonElement::from_type_slice(JsonType::JsonString, slice);
    }

    pub fn from_number(number: JsonNumber, slice: Slice<'a>) -> JsonElement<'a> {
        return JsonElement {
            json_type: JsonType::JsonNumber,
            slice,
            boolean: None,
            number: Some(number),
            object: None,
            array: None,
        };
    }

    pub fn from_object(object: JsonObject<'a>, slice: Slice<'a>) -> JsonElement<'a> {
        return JsonElement {
            json_type: JsonType::JsonObject,
            slice,
            boolean: None,
            number: None,
            object: Some(object),
            array: None,
        };
    }

    pub fn from_array(array: JsonArray<'a>, slice: Slice<'a>) -> JsonElement<'a> {
        return JsonElement {
            json_type: JsonType::JsonArray,
            slice,
            boolean: None,
            number: None,
            object: None,
            array: Some(array),
        };
    }

    pub const fn empty() -> JsonElement<'a> {
        return JsonElement {
            json_type: JsonType::JsonEmpty,
            slice: Slice {
                source: &[],
                beginning: 0,
                end: 0,
            },
            boolean: None,
            number: None,
            object: None,
            array: None,
        };
    }

    pub fn exists(&self) -> bool {
        return self.json_type != JsonEmpty;
    }

    pub fn as_str(&self) -> Option<&str> {
        match self.json_type {
            JsonType::JsonEmpty => { None }
            _ => { Some(self.get_slice().as_str()) }
        }
    }

    pub fn as_i128(&self) -> Option<i128> {
        match self.json_type {
            JsonType::JsonNumber => { Some(self.number.as_ref().unwrap().i128()) }
            _ => { None }
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self.json_type {
            JsonType::JsonNumber => { Some(self.number.as_ref().unwrap().f64()) }
            _ => { None }
        }
    }
}

impl<'a> Index<&'a str> for JsonElement<'a> {
    type Output = JsonElement<'a>;

    fn index(&self, key: &str) -> &Self::Output {
        return match self.json_type {
            JsonType::JsonObject => {
                return self.object.as_ref().unwrap().map.get(key).unwrap_or(&EMPTY_ELEMENT);
            }
            _ => {
                &EMPTY_ELEMENT
            }
        };
    }
}

impl<'a> Index<usize> for JsonElement<'a> {
    type Output = JsonElement<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        return match self.json_type {
            JsonType::JsonArray => {
                return self.array.as_ref().unwrap().vec.get(index).unwrap_or(&EMPTY_ELEMENT);
            }
            _ => {
                &EMPTY_ELEMENT
            }
        };
    }
}