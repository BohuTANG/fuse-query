// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::fmt;

use anyhow::bail;
use anyhow::Result;
use common_datablocks::DataBlock;
use common_datavalues::DataColumnarValue;
use common_datavalues::DataSchema;
use common_datavalues::DataType;
use common_datavalues::DataValue;
use dyn_clone::DynClone;

pub trait IFunction: fmt::Display + Sync + Send + DynClone {
    fn name(&self) -> &str;
    fn return_type(&self, input_schema: &DataSchema) -> Result<DataType>;
    fn nullable(&self, input_schema: &DataSchema) -> Result<bool>;
    fn eval(&self, block: &DataBlock) -> Result<DataColumnarValue>;
    fn set_depth(&mut self, _depth: usize) {}

    fn accumulate(&mut self, _block: &DataBlock) -> Result<()> {
        bail!("Function Error: '{}' accumulate unimplemented", self.name());
    }

    fn accumulate_result(&self) -> Result<Vec<DataValue>> {
        bail!(
            "Function Error: '{}' accumulate_result unimplemented",
            self.name()
        );
    }

    fn merge(&mut self, _states: &[DataValue]) -> Result<()> {
        bail!("Function Error: '{}' merge unimplemented", self.name());
    }

    fn merge_result(&self) -> Result<DataValue> {
        bail!(
            "Function Error: '{}' merge_result unimplemented",
            self.name()
        );
    }

    fn is_aggregator(&self) -> bool {
        false
    }
}

dyn_clone::clone_trait_object!(IFunction);
