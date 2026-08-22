//! Projection Operator — SELECT column selection and computation

use crate::error::ByteRagResult;
use crate::sql::executor::operators::PhysicalOperator;
use crate::sql::planner::PhysicalExpr;
use arrow::array::ArrayRef;
use arrow::datatypes::Schema;
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

/// Projection 연산자 (SELECT 컬럼 선택/계산)
pub struct ProjectionOperator {
    input: Box<dyn PhysicalOperator>,
    schema: Arc<Schema>,
    /// Expressions to evaluate for each output column
    exprs: Vec<PhysicalExpr>,
}

impl ProjectionOperator {
    pub fn new(
        input: Box<dyn PhysicalOperator>,
        schema: Arc<Schema>,
        exprs: Vec<PhysicalExpr>,
    ) -> Self {
        Self {
            input,
            schema,
            exprs,
        }
    }
}

impl PhysicalOperator for ProjectionOperator {
    fn schema(&self) -> &Schema {
        &self.schema
    }

    fn next(&mut self) -> ByteRagResult<Option<RecordBatch>> {
        match self.input.next()? {
            None => Ok(None),
            Some(batch) => {
                if self.exprs.is_empty() {
                    // SELECT * — pass through all columns
                    return Ok(Some(batch));
                }

                let columns: Vec<ArrayRef> = self
                    .exprs
                    .iter()
                    .map(|expr| super::super::evaluate_expr(expr, &batch))
                    .collect::<ByteRagResult<_>>()?;

                Ok(Some(RecordBatch::try_new(
                    Arc::clone(&self.schema),
                    columns,
                )?))
            }
        }
    }

    fn reset(&mut self) -> ByteRagResult<()> {
        self.input.reset()
    }
}
