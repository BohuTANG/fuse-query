// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_setting_interpreter() -> anyhow::Result<()> {
    use common_planners::*;
    use futures::stream::StreamExt;
    use pretty_assertions::assert_eq;

    use crate::interpreters::*;
    use crate::sql::*;

    let ctx = crate::tests::try_create_context()?;

    if let PlanNode::SetVariable(plan) =
        PlanParser::create(ctx.clone()).build_from_sql("set max_block_size=1")?
    {
        let executor = SettingInterpreter::try_create(ctx, plan)?;
        assert_eq!(executor.name(), "SettingInterpreter");

        let mut stream = executor.execute().await?;
        while let Some(_block) = stream.next().await {}
    } else {
        assert!(false)
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_setting_interpreter_error() -> anyhow::Result<()> {
    use common_planners::*;
    use pretty_assertions::assert_eq;

    use crate::interpreters::*;
    use crate::sql::*;

    let ctx = crate::tests::try_create_context()?;

    if let PlanNode::SetVariable(plan) =
        PlanParser::create(ctx.clone()).build_from_sql("set xx=1")?
    {
        let executor = SettingInterpreter::try_create(ctx, plan)?;
        if let Err(e) = executor.execute().await {
            let expect = "Unknown variable: \"xx\"";
            let actual = format!("{:?}", e);
            assert_eq!(expect, actual);
        } else {
            assert!(false);
        }
    }

    Ok(())
}
