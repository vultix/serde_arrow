use super::type_support::FieldRef;
use crate::{
    _impl::arrow::datatypes::{DataType, Field, UnionMode},
    internal::{
        error::{error, fail, Error, Result},
        schema::{GenericDataType, GenericField, Strategy, STRATEGY_KEY},
    },
};

impl TryFrom<&DataType> for GenericDataType {
    type Error = Error;

    fn try_from(value: &DataType) -> Result<GenericDataType> {
        match value {
            DataType::Boolean => Ok(GenericDataType::Bool),
            DataType::Null => Ok(GenericDataType::Null),
            DataType::Int8 => Ok(GenericDataType::I8),
            DataType::Int16 => Ok(GenericDataType::I16),
            DataType::Int32 => Ok(GenericDataType::I32),
            DataType::Int64 => Ok(GenericDataType::I64),
            DataType::UInt8 => Ok(GenericDataType::U8),
            DataType::UInt16 => Ok(GenericDataType::U16),
            DataType::UInt32 => Ok(GenericDataType::U32),
            DataType::UInt64 => Ok(GenericDataType::U64),
            DataType::Float16 => Ok(GenericDataType::F16),
            DataType::Float32 => Ok(GenericDataType::F32),
            DataType::Float64 => Ok(GenericDataType::F64),
            DataType::Utf8 => Ok(GenericDataType::Utf8),
            DataType::LargeUtf8 => Ok(GenericDataType::LargeUtf8),
            DataType::Date64 => Ok(GenericDataType::Date64),
            _ => fail!("Only primitive data types can be converted to GenericDataType"),
        }
    }
}

impl TryFrom<&Field> for GenericField {
    type Error = Error;

    fn try_from(field: &Field) -> Result<Self> {
        let strategy: Option<Strategy> = match field.metadata().get(STRATEGY_KEY) {
            Some(strategy_str) => Some(strategy_str.parse::<Strategy>()?),
            None => None,
        };
        let name = field.name().to_owned();
        let nullable = field.is_nullable();

        let mut children = Vec::<GenericField>::new();
        let data_type = match field.data_type() {
            DataType::List(field) => {
                children.push(GenericField::try_from(field.as_ref())?);
                GenericDataType::List
            }
            DataType::LargeList(field) => {
                children.push(field.as_ref().try_into()?);
                GenericDataType::LargeList
            }
            DataType::Struct(fields) => {
                for field in fields {
                    children.push(field.as_field_ref().try_into()?);
                }
                GenericDataType::Struct
            }
            DataType::Map(field, _) => {
                children.push(field.as_ref().try_into()?);
                GenericDataType::Map
            }
            #[cfg(not(any(feature = "arrow-35", feature = "arrow-36")))]
            DataType::Union(fields, mode) => {
                if !matches!(mode, UnionMode::Dense) {
                    fail!("Only dense unions are supported at the moment");
                }

                for (pos, (idx, field)) in fields.iter().enumerate() {
                    if pos as i8 != idx {
                        fail!("Union types with explicit field indices are not supported");
                    }
                    children.push(field.as_ref().try_into()?);
                }
                GenericDataType::Union
            }
            #[cfg(any(feature = "arrow-35", feature = "arrow-36"))]
            DataType::Union(fields, field_indices, mode) => {
                if field_indices
                    .iter()
                    .enumerate()
                    .any(|(pos, &idx)| idx < 0 || pos != (idx as usize))
                {
                    fail!("Union types with explicit field indices are not supported");
                }
                if !matches!(mode, UnionMode::Dense) {
                    fail!("Only dense unions are supported at the moment");
                }

                for field in fields {
                    children.push(field.try_into()?);
                }
                GenericDataType::Union
            }
            DataType::Dictionary(key_type, value_type) => {
                children.push(GenericField::new("", key_type.as_ref().try_into()?, false));
                children.push(GenericField::new(
                    "",
                    value_type.as_ref().try_into()?,
                    false,
                ));
                GenericDataType::Dictionary
            }
            dt => dt.try_into()?,
        };

        Ok(GenericField {
            data_type,
            name,
            strategy,
            children,
            nullable,
        })
    }
}

impl TryFrom<&GenericField> for Field {
    type Error = Error;

    fn try_from(value: &GenericField) -> Result<Self> {
        let data_type = match &value.data_type {
            GenericDataType::Null => DataType::Null,
            GenericDataType::Bool => DataType::Boolean,
            GenericDataType::I8 => DataType::Int8,
            GenericDataType::I16 => DataType::Int16,
            GenericDataType::I32 => DataType::Int32,
            GenericDataType::I64 => DataType::Int64,
            GenericDataType::U8 => DataType::UInt8,
            GenericDataType::U16 => DataType::UInt16,
            GenericDataType::U32 => DataType::UInt32,
            GenericDataType::U64 => DataType::UInt64,
            GenericDataType::F16 => DataType::Float16,
            GenericDataType::F32 => DataType::Float32,
            GenericDataType::F64 => DataType::Float64,
            GenericDataType::Date64 => DataType::Date64,
            GenericDataType::Utf8 => DataType::Utf8,
            GenericDataType::LargeUtf8 => DataType::LargeUtf8,
            GenericDataType::List => DataType::List(
                Box::<Field>::new(
                    value
                        .children
                        .get(0)
                        .ok_or_else(|| error!("List must a single child"))?
                        .try_into()?,
                )
                .into(),
            ),
            GenericDataType::LargeList => DataType::LargeList(
                Box::<Field>::new(
                    value
                        .children
                        .get(0)
                        .ok_or_else(|| error!("List must a single child"))?
                        .try_into()?,
                )
                .into(),
            ),
            GenericDataType::Struct => DataType::Struct(
                value
                    .children
                    .iter()
                    .map(Field::try_from)
                    .collect::<Result<_>>()?,
            ),
            GenericDataType::Map => {
                let element_field: Field = value
                    .children
                    .get(0)
                    .ok_or_else(|| error!("Map must a single child"))?
                    .try_into()?;
                DataType::Map(Box::new(element_field).into(), false)
            }
            #[cfg(not(any(feature = "arrow-35", feature = "arrow-36")))]
            GenericDataType::Union => {
                let mut fields = Vec::new();
                for (idx, field) in value.children.iter().enumerate() {
                    fields.push((idx as i8, std::sync::Arc::new(Field::try_from(field)?)));
                }
                DataType::Union(fields.into_iter().collect(), UnionMode::Dense)
            }
            #[cfg(any(feature = "arrow-35", feature = "arrow-36"))]
            GenericDataType::Union => DataType::Union(
                value
                    .children
                    .iter()
                    .map(Field::try_from)
                    .collect::<Result<Vec<_>>>()?,
                (0..value.children.len())
                    .into_iter()
                    .map(|v| v as i8)
                    .collect(),
                UnionMode::Dense,
            ),
            GenericDataType::Dictionary => {
                let key_field = value
                    .children
                    .get(0)
                    .ok_or_else(|| error!("Dictionary must a two children"))?;
                let val_field: Field = value
                    .children
                    .get(1)
                    .ok_or_else(|| error!("Dictionary must a two children"))?
                    .try_into()?;

                let key_type = match &key_field.data_type {
                    GenericDataType::U8 => DataType::UInt8,
                    GenericDataType::U16 => DataType::UInt16,
                    GenericDataType::U32 => DataType::UInt32,
                    GenericDataType::U64 => DataType::UInt64,
                    GenericDataType::I8 => DataType::Int8,
                    GenericDataType::I16 => DataType::Int16,
                    GenericDataType::I32 => DataType::Int32,
                    GenericDataType::I64 => DataType::Int64,
                    _ => fail!("Invalid key type for dictionary"),
                };

                DataType::Dictionary(Box::new(key_type), Box::new(val_field.data_type().clone()))
            }
        };

        let mut field = Field::new(&value.name, data_type, value.nullable);
        if let Some(strategy) = value.strategy.as_ref() {
            field.set_metadata(strategy.clone().into());
        }

        Ok(field)
    }
}
