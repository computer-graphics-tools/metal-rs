use objc2::{Encode, Encoding, RefEncode};

use crate::types::DataType;

/// The possible data types for the elements of a tensor (from `MTLTensorDataType`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum TensorDataType {
    /// None
    None = DataType::None as i64,
    /// 32-bit float
    Float32 = DataType::Float as i64,
    /// 16-bit float
    Float16 = DataType::Half as i64,
    /// 16-bit bfloat
    BFloat16 = DataType::BFloat as i64,
    /// 8-bit signed integer
    Int8 = DataType::Char as i64,
    /// 8-bit unsigned integer
    UInt8 = DataType::UChar as i64,
    /// 16-bit signed integer
    Int16 = DataType::Short as i64,
    /// 16-bit unsigned integer
    UInt16 = DataType::UShort as i64,
    /// 32-bit signed integer
    Int32 = DataType::Int as i64,
    /// 32-bit unsigned integer
    UInt32 = DataType::UInt as i64,
}

unsafe impl Encode for TensorDataType {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for TensorDataType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
