use crate::semantic::{SemanticData, Type};

impl Type {
    pub fn alignment(&self, semantics: &SemanticData) -> u8 {
        match self {
            Type::Char | Type::SChar | Type::UChar => 1,
            Type::Int | Type::UInt => 4,
            Type::Long | Type::ULong | Type::Double | Type::Pointer(_) => 8,
            Type::Array(inner, _) => inner.alignment(semantics),
            Type::Struct(name) | Type::Union(name) => {
                semantics.get_aggregate(name).alignment
            }
            Type::Function(_) => panic!("Function type does not have alignment"),
            Type::Void => panic!("Void does not have alignment"),
        }
    }

    pub fn size(&self, semantics: &SemanticData) -> usize {
        match self {
            Type::Char | Type::UChar | Type::SChar => 1,
            Type::Int => 4,
            Type::UInt => 4,
            Type::Long => 8,
            Type::ULong => 8,
            Type::Double => 8,
            Type::Function(_) => panic!("Size of a function type"),
            Type::Pointer(_) => 8,
            Type::Array(ty, size) => ty.size(semantics) * size,
            Type::Void => 1,
            Type::Struct(name) | Type::Union(name) => semantics.get_aggregate(name).size,
        }
    }
}

pub fn align_offset(unaligned_offset: usize, alignment: u8) -> usize {
    let alignment = alignment as usize;
    // Alignment formula found on the internet
    (unaligned_offset + alignment - 1) & !(alignment - 1)
}
