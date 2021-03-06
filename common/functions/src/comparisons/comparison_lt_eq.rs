// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use anyhow::Result;
use common_datavalues::DataValueComparisonOperator;

use crate::comparisons::ComparisonFunction;
use crate::IFunction;

pub struct ComparisonLtEqFunction;

impl ComparisonLtEqFunction {
    pub fn try_create_func(args: &[Box<dyn IFunction>]) -> Result<Box<dyn IFunction>> {
        ComparisonFunction::try_create_func(DataValueComparisonOperator::LtEq, args)
    }
}
