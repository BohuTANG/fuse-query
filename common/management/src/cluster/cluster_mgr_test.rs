// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use common_exception::Result;
use common_runtime::tokio;
use pretty_assertions::assert_eq;

use crate::cluster::address::Address;
use crate::ClusterExecutor;
use crate::ClusterMgr;

#[tokio::test]
async fn test_cluster_mgr() -> Result<()> {
    let executor1 = ClusterExecutor {
        name: "n1".to_string(),
        priority: 0,
        address: Address::create("192.168.0.1:9091")?,
        local: false,
        sequence: 0,
    };
    let executor2 = ClusterExecutor {
        name: "n2".to_string(),
        priority: 0,
        address: Address::create("192.168.0.2:9091")?,
        local: false,
        sequence: 0,
    };
    let namespace = "namespace-1".to_string();
    let mut cluster_mgr = ClusterMgr::create("".to_string());

    // Register.
    {
        cluster_mgr.register(namespace.clone(), &executor1).await?;
        cluster_mgr.register(namespace.clone(), &executor2).await?;
        cluster_mgr.register(namespace.clone(), &executor1).await?;
        cluster_mgr.register(namespace.clone(), &executor2).await?;

        let actual = cluster_mgr.get_executors(namespace.clone()).await?;
        let expect = vec![executor1.clone(), executor2.clone()];
        assert_eq!(actual, expect);
    }

    // Unregister.
    {
        cluster_mgr
            .unregister(namespace.clone(), &executor1)
            .await?;
        cluster_mgr
            .unregister(namespace.clone(), &executor1)
            .await?;

        let actual = cluster_mgr.get_executors(namespace).await?;
        let expect = vec![executor2.clone()];
        assert_eq!(actual, expect);
    }

    Ok(())
}