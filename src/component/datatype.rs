#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Char(u8),
    Double,
    Float,
    Int,
    Varchar(u8),
    Url,
}

impl DataType {
    pub fn get(s: &str, len: Option<u8>) -> Option<DataType> {
        let length = len.unwrap_or(0);
        let d = match s {
            "char" => DataType::Char(length),
            "double" => DataType::Double,
            "float" => DataType::Float,
            "int" => DataType::Int,
            "varchar" => DataType::Varchar(length),
            "url" => DataType::Url,
            _ => return None,
        };
        Some(d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datatype() {
        assert_eq!(DataType::Double, DataType::get("double", None).unwrap());
        assert_eq!(DataType::Char(8), DataType::get("char", Some(8)).unwrap());
        assert_eq!(DataType::Url, DataType::get("url", None).unwrap());
        assert!(DataType::get("date", None).is_none());
    }
}
