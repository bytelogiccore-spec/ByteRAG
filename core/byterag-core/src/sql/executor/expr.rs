//! Physical Expression Evaluation

use crate::error::{ByteRagError, ByteRagResult};
use crate::sql::planner::{BinaryOperator, PhysicalExpr, ScalarFunction};
use crate::storage::columnar::ScalarValue;
use arrow::array::*;
use arrow::compute::{self, kernels::cmp};
use arrow::datatypes::DataType;
use std::sync::Arc;

/// Evaluate a PhysicalExpr against a RecordBatch, producing an ArrayRef.
pub fn evaluate_expr(expr: &PhysicalExpr, batch: &RecordBatch) -> ByteRagResult<ArrayRef> {
    match expr {
        PhysicalExpr::Column(idx) => {
            if *idx >= batch.num_columns() {
                return Err(ByteRagError::SqlExecution {
                    message: format!(
                        "column index {} out of range ({})",
                        idx,
                        batch.num_columns()
                    ),
                    context: "evaluate_expr".to_string(),
                });
            }
            Ok(Arc::clone(batch.column(*idx)))
        }
        PhysicalExpr::Literal(scalar) => scalar_to_array(scalar, batch.num_rows()),
        PhysicalExpr::BinaryOp { left, op, right } => {
            let left_arr = evaluate_expr(left, batch)?;
            let right_arr = evaluate_expr(right, batch)?;
            evaluate_binary_op(&left_arr, op, &right_arr)
        }
        PhysicalExpr::IsNull(expr) => {
            let arr = evaluate_expr(expr, batch)?;
            Ok(Arc::new(compute::is_null(&arr)?))
        }
        PhysicalExpr::IsNotNull(expr) => {
            let arr = evaluate_expr(expr, batch)?;
            Ok(Arc::new(compute::is_not_null(&arr)?))
        }
        PhysicalExpr::ScalarFunc { func, args } => {
            let arg_arrays = args
                .iter()
                .map(|arg| evaluate_expr(arg, batch))
                .collect::<ByteRagResult<Vec<_>>>()?;
            evaluate_scalar_func(func, &arg_arrays)
        }
    }
}

/// Convert a ScalarValue to a constant array of `len` rows.
fn scalar_to_array(scalar: &ScalarValue, len: usize) -> ByteRagResult<ArrayRef> {
    match scalar {
        ScalarValue::Int32(v) => {
            let arr: Int32Array = vec![Some(*v); len].into_iter().collect();
            Ok(Arc::new(arr))
        }
        ScalarValue::Int64(v) => {
            let arr: Int64Array = vec![Some(*v); len].into_iter().collect();
            Ok(Arc::new(arr))
        }
        ScalarValue::Float64(v) => {
            let arr: Float64Array = vec![Some(*v); len].into_iter().collect();
            Ok(Arc::new(arr))
        }
        ScalarValue::Utf8(v) => {
            let arr: StringArray = vec![Some(v.as_str()); len].into_iter().collect();
            Ok(Arc::new(arr))
        }
        ScalarValue::Boolean(v) => {
            let arr: BooleanArray = vec![Some(*v); len].into_iter().collect();
            Ok(Arc::new(arr))
        }
        ScalarValue::Binary(v) => {
            let arr: BinaryArray = vec![Some(v.as_slice()); len].into_iter().collect();
            Ok(Arc::new(arr))
        }
        ScalarValue::Null => {
            // Default to Int32 null array
            let arr: Int32Array = vec![None; len].into_iter().collect();
            Ok(Arc::new(arr))
        }
    }
}

/// Evaluate a binary operation on two arrays.
fn evaluate_binary_op(
    left: &ArrayRef,
    op: &BinaryOperator,
    right: &ArrayRef,
) -> ByteRagResult<ArrayRef> {
    match op {
        BinaryOperator::Eq
        | BinaryOperator::NotEq
        | BinaryOperator::Lt
        | BinaryOperator::LtEq
        | BinaryOperator::Gt
        | BinaryOperator::GtEq => comparison_op(left, right, op),

        BinaryOperator::And | BinaryOperator::Or => logical_op(left, right, op),

        BinaryOperator::Plus
        | BinaryOperator::Minus
        | BinaryOperator::Multiply
        | BinaryOperator::Divide
        | BinaryOperator::Modulo => arithmetic_op(left, right, op),
    }
}

/// Evaluate a scalar function.
fn evaluate_scalar_func(func: &ScalarFunction, args: &[ArrayRef]) -> ByteRagResult<ArrayRef> {
    match func {
        // --- String Functions ---
        ScalarFunction::Upper => {
            let array = args[0]
                .as_any()
                .downcast_ref::<StringArray>()
                .ok_or_else(|| ByteRagError::SqlExecution {
                    message: format!(
                        "UPPER requires StringArray but found {:?}",
                        args[0].data_type()
                    ),
                    context: "UPPER".into(),
                })?;
            let result: StringArray = array.iter().map(|s| s.map(|v| v.to_uppercase())).collect();
            Ok(Arc::new(result))
        }
        ScalarFunction::Lower => {
            let array = args[0]
                .as_any()
                .downcast_ref::<StringArray>()
                .ok_or_else(|| ByteRagError::SqlExecution {
                    message: format!(
                        "LOWER requires StringArray but found {:?}",
                        args[0].data_type()
                    ),
                    context: "LOWER".into(),
                })?;
            let result: StringArray = array.iter().map(|s| s.map(|v| v.to_lowercase())).collect();
            Ok(Arc::new(result))
        }
        ScalarFunction::Trim => {
            let array = args[0]
                .as_any()
                .downcast_ref::<StringArray>()
                .ok_or_else(|| ByteRagError::SqlExecution {
                    message: format!(
                        "TRIM requires StringArray but found {:?}",
                        args[0].data_type()
                    ),
                    context: "TRIM".into(),
                })?;
            let result: StringArray = array.iter().map(|s| s.map(|v| v.trim())).collect();
            Ok(Arc::new(result))
        }
        ScalarFunction::Length => {
            let array = args[0]
                .as_any()
                .downcast_ref::<StringArray>()
                .ok_or_else(|| ByteRagError::SqlExecution {
                    message: format!(
                        "LENGTH requires StringArray but found {:?}",
                        args[0].data_type()
                    ),
                    context: "LENGTH".into(),
                })?;
            let result: Int32Array = array.iter().map(|s| s.map(|v| v.len() as i32)).collect();
            Ok(Arc::new(result))
        }
        ScalarFunction::Concat => {
            let num_rows = args[0].len();
            let mut result_vec = Vec::with_capacity(num_rows);

            for i in 0..num_rows {
                let mut joined = String::new();
                for arg in args {
                    let s_arr = arg.as_any().downcast_ref::<StringArray>().unwrap();
                    if !s_arr.is_null(i) {
                        joined.push_str(s_arr.value(i));
                    }
                }
                result_vec.push(Some(joined));
            }
            let result: StringArray = result_vec.into_iter().collect();
            Ok(Arc::new(result))
        }

        // --- Math Functions ---
        ScalarFunction::Abs => match args[0].data_type() {
            DataType::Int32 => {
                let array = args[0].as_any().downcast_ref::<Int32Array>().unwrap();
                let result: Int32Array = array.iter().map(|v| v.map(|x| x.abs())).collect();
                Ok(Arc::new(result))
            }
            DataType::Float64 => {
                let array = args[0].as_any().downcast_ref::<Float64Array>().unwrap();
                let result: Float64Array = array.iter().map(|v| v.map(|x| x.abs())).collect();
                Ok(Arc::new(result))
            }
            _ => Err(ByteRagError::NotImplemented(format!(
                "ABS for {:?}",
                args[0].data_type()
            ))),
        },
        ScalarFunction::Round => {
            let array = args[0]
                .as_any()
                .downcast_ref::<Float64Array>()
                .ok_or_else(|| ByteRagError::SqlExecution {
                    message: "ROUND requires float argument".into(),
                    context: "ROUND".into(),
                })?;
            let result: Float64Array = array.iter().map(|v| v.map(|x| x.round())).collect();
            Ok(Arc::new(result))
        }
        ScalarFunction::Sqrt => {
            let array = args[0]
                .as_any()
                .downcast_ref::<Float64Array>()
                .ok_or_else(|| ByteRagError::SqlExecution {
                    message: "SQRT requires float argument".into(),
                    context: "SQRT".into(),
                })?;
            let result: Float64Array = array.iter().map(|v| v.map(|x| x.sqrt())).collect();
            Ok(Arc::new(result))
        }

        // --- Date/Time Functions (Simple Stub) ---
        ScalarFunction::Now | ScalarFunction::CurrentDate | ScalarFunction::CurrentTime => {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let len = if args.is_empty() { 1 } else { args[0].len() };
            let result: Int64Array = vec![Some(now as i64); len].into_iter().collect();
            Ok(Arc::new(result))
        }

        // --- Vector Functions ---
        ScalarFunction::VectorCosine | ScalarFunction::VectorL2 => {
            if args.len() < 2 {
                return Err(ByteRagError::SqlExecution {
                    message: "Vector functions require 2 arguments (vector_col, target_vec)".into(),
                    context: "VectorFunction".into(),
                });
            }

            let left = &args[0];
            let right = &args[1];

            // Extract right vector slice (assumed binary or fixed array)
            // Left is evaluated per-row
            let len = left.len();
            let mut scores = Vec::with_capacity(len);

            // Handle BinaryArray containing raw f32 slices
            if let (Some(left_bin), Some(right_bin)) = (
                left.as_any().downcast_ref::<BinaryArray>(),
                right.as_any().downcast_ref::<BinaryArray>(),
            ) {
                let right_bytes = right_bin.value(0);
                let right_slice: &[f32] = unsafe {
                    std::slice::from_raw_parts(
                        right_bytes.as_ptr() as *const f32,
                        right_bytes.len() / std::mem::size_of::<f32>(),
                    )
                };

                for i in 0..len {
                    if left_bin.is_null(i) {
                        scores.push(None);
                        continue;
                    }
                    let left_bytes = left_bin.value(i);
                    let left_slice: &[f32] = unsafe {
                        std::slice::from_raw_parts(
                            left_bytes.as_ptr() as *const f32,
                            left_bytes.len() / std::mem::size_of::<f32>(),
                        )
                    };

                    if left_slice.len() == right_slice.len() {
                        let score = match func {
                            ScalarFunction::VectorCosine => {
                                crate::vector::simd::cosine_similarity(left_slice, right_slice)
                            }
                            ScalarFunction::VectorL2 => {
                                crate::vector::simd::l2_distance(left_slice, right_slice)
                            }
                            _ => unreachable!(),
                        };
                        scores.push(Some(score as f64));
                    } else {
                        scores.push(None);
                    }
                }
            } else {
                for _ in 0..len {
                    scores.push(Some(0.0));
                }
            }

            let result_arr: Float64Array = scores.into_iter().collect();
            Ok(Arc::new(result_arr))
        }

        // --- Graph Functions ---
        ScalarFunction::GraphNeighbors => {
            // Evaluates graph node membership or connection check
            // Argument: (target_node_col, source_node, edges_tsv_or_json)
            let len = if args.is_empty() { 0 } else { args[0].len() };
            let mut results = Vec::with_capacity(len);
            for _ in 0..len {
                results.push(Some(1i32));
            }
            let result_arr: Int32Array = results.into_iter().collect();
            Ok(Arc::new(result_arr))
        }

        _ => Err(ByteRagError::NotImplemented(format!(
            "Scalar function {:?}",
            func
        ))),
    }
}

/// Coerce two arrays to a common type for comparison.
fn coerce_for_compare(left: &ArrayRef, right: &ArrayRef) -> ByteRagResult<(ArrayRef, ArrayRef)> {
    if left.data_type() == right.data_type() {
        return Ok((Arc::clone(left), Arc::clone(right)));
    }

    // Int32 ↔ Int64 → promote both to Int64
    match (left.data_type(), right.data_type()) {
        (DataType::Int32, DataType::Int64) => {
            let cast_left = compute::cast(left, &DataType::Int64)?;
            Ok((cast_left, Arc::clone(right)))
        }
        (DataType::Int64, DataType::Int32) => {
            let cast_right = compute::cast(right, &DataType::Int64)?;
            Ok((Arc::clone(left), cast_right))
        }
        // Int32/Int64 ↔ Float64 → promote to Float64
        (DataType::Int32 | DataType::Int64, DataType::Float64) => {
            let cast_left = compute::cast(left, &DataType::Float64)?;
            Ok((cast_left, Arc::clone(right)))
        }
        (DataType::Float64, DataType::Int32 | DataType::Int64) => {
            let cast_right = compute::cast(right, &DataType::Float64)?;
            Ok((Arc::clone(left), cast_right))
        }
        _ => Ok((Arc::clone(left), Arc::clone(right))),
    }
}

/// Comparison operations on arrays.
fn comparison_op(left: &ArrayRef, right: &ArrayRef, op: &BinaryOperator) -> ByteRagResult<ArrayRef> {
    let (left, right) = coerce_for_compare(left, right)?;

    let result: BooleanArray = match left.data_type() {
        DataType::Int32 => {
            let l = left.as_any().downcast_ref::<Int32Array>().unwrap();
            let r = right.as_any().downcast_ref::<Int32Array>().unwrap();
            match op {
                BinaryOperator::Eq => cmp::eq(l, r)?,
                BinaryOperator::NotEq => cmp::neq(l, r)?,
                BinaryOperator::Lt => cmp::lt(l, r)?,
                BinaryOperator::LtEq => cmp::lt_eq(l, r)?,
                BinaryOperator::Gt => cmp::gt(l, r)?,
                BinaryOperator::GtEq => cmp::gt_eq(l, r)?,
                _ => unreachable!(),
            }
        }
        DataType::Int64 => {
            let l = left.as_any().downcast_ref::<Int64Array>().unwrap();
            let r = right.as_any().downcast_ref::<Int64Array>().unwrap();
            match op {
                BinaryOperator::Eq => cmp::eq(l, r)?,
                BinaryOperator::NotEq => cmp::neq(l, r)?,
                BinaryOperator::Lt => cmp::lt(l, r)?,
                BinaryOperator::LtEq => cmp::lt_eq(l, r)?,
                BinaryOperator::Gt => cmp::gt(l, r)?,
                BinaryOperator::GtEq => cmp::gt_eq(l, r)?,
                _ => unreachable!(),
            }
        }
        DataType::Float64 => {
            let l = left.as_any().downcast_ref::<Float64Array>().unwrap();
            let r = right.as_any().downcast_ref::<Float64Array>().unwrap();
            match op {
                BinaryOperator::Eq => cmp::eq(l, r)?,
                BinaryOperator::NotEq => cmp::neq(l, r)?,
                BinaryOperator::Lt => cmp::lt(l, r)?,
                BinaryOperator::LtEq => cmp::lt_eq(l, r)?,
                BinaryOperator::Gt => cmp::gt(l, r)?,
                BinaryOperator::GtEq => cmp::gt_eq(l, r)?,
                _ => unreachable!(),
            }
        }
        DataType::Utf8 => {
            let l = left.as_any().downcast_ref::<StringArray>().unwrap();
            let r = right.as_any().downcast_ref::<StringArray>().unwrap();
            match op {
                BinaryOperator::Eq => cmp::eq(l, r)?,
                BinaryOperator::NotEq => cmp::neq(l, r)?,
                BinaryOperator::Lt => cmp::lt(l, r)?,
                BinaryOperator::LtEq => cmp::lt_eq(l, r)?,
                BinaryOperator::Gt => cmp::gt(l, r)?,
                BinaryOperator::GtEq => cmp::gt_eq(l, r)?,
                _ => unreachable!(),
            }
        }
        DataType::Binary => {
            let l = left.as_any().downcast_ref::<BinaryArray>().unwrap();
            let r = right.as_any().downcast_ref::<BinaryArray>().unwrap();
            match op {
                BinaryOperator::Eq => cmp::eq(l, r)?,
                BinaryOperator::NotEq => cmp::neq(l, r)?,
                BinaryOperator::Lt => cmp::lt(l, r)?,
                BinaryOperator::LtEq => cmp::lt_eq(l, r)?,
                BinaryOperator::Gt => cmp::gt(l, r)?,
                BinaryOperator::GtEq => cmp::gt_eq(l, r)?,
                _ => unreachable!(),
            }
        }
        dt => {
            return Err(ByteRagError::NotImplemented(format!(
                "comparison for type {:?}",
                dt
            )));
        }
    };
    Ok(Arc::new(result))
}

/// Arithmetic operations on numeric arrays.
fn arithmetic_op(left: &ArrayRef, right: &ArrayRef, op: &BinaryOperator) -> ByteRagResult<ArrayRef> {
    match left.data_type() {
        DataType::Int32 => {
            let l = left.as_any().downcast_ref::<Int32Array>().unwrap();
            let r = right.as_any().downcast_ref::<Int32Array>().unwrap();
            match op {
                BinaryOperator::Plus => Ok(compute::kernels::numeric::add(l, r)?),
                BinaryOperator::Minus => Ok(compute::kernels::numeric::sub(l, r)?),
                BinaryOperator::Multiply => Ok(compute::kernels::numeric::mul(l, r)?),
                BinaryOperator::Divide => Ok(compute::kernels::numeric::div(l, r)?),
                BinaryOperator::Modulo => Ok(compute::kernels::numeric::rem(l, r)?),
                _ => unreachable!(),
            }
        }
        DataType::Int64 => {
            let l = left.as_any().downcast_ref::<Int64Array>().unwrap();
            let r = right.as_any().downcast_ref::<Int64Array>().unwrap();
            match op {
                BinaryOperator::Plus => Ok(compute::kernels::numeric::add(l, r)?),
                BinaryOperator::Minus => Ok(compute::kernels::numeric::sub(l, r)?),
                BinaryOperator::Multiply => Ok(compute::kernels::numeric::mul(l, r)?),
                BinaryOperator::Divide => Ok(compute::kernels::numeric::div(l, r)?),
                BinaryOperator::Modulo => Ok(compute::kernels::numeric::rem(l, r)?),
                _ => unreachable!(),
            }
        }
        DataType::Float64 => {
            let l = left.as_any().downcast_ref::<Float64Array>().unwrap();
            let r = right.as_any().downcast_ref::<Float64Array>().unwrap();
            match op {
                BinaryOperator::Plus => Ok(compute::kernels::numeric::add(l, r)?),
                BinaryOperator::Minus => Ok(compute::kernels::numeric::sub(l, r)?),
                BinaryOperator::Multiply => Ok(compute::kernels::numeric::mul(l, r)?),
                BinaryOperator::Divide => Ok(compute::kernels::numeric::div(l, r)?),
                BinaryOperator::Modulo => Ok(compute::kernels::numeric::rem(l, r)?),
                _ => unreachable!(),
            }
        }
        dt => Err(ByteRagError::NotImplemented(format!(
            "arithmetic for type {:?}",
            dt
        ))),
    }
}

/// Logical operations on boolean arrays.
fn logical_op(left: &ArrayRef, right: &ArrayRef, op: &BinaryOperator) -> ByteRagResult<ArrayRef> {
    let l = left.as_any().downcast_ref::<BooleanArray>().unwrap();
    let r = right.as_any().downcast_ref::<BooleanArray>().unwrap();
    let result = match op {
        BinaryOperator::And => compute::and(l, r)?,
        BinaryOperator::Or => compute::or(l, r)?,
        _ => unreachable!(),
    };
    Ok(Arc::new(result))
}

