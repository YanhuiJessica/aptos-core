// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::metrics::{BIG_QUERY_REQUEST_FAILURES_TOTAL, BIG_QUERY_REQUEST_TOTAL};
use aptos_infallible::RwLock;
use aptos_types::PeerId;
use gcp_bigquery_client::{model::query_request::QueryRequest, Client as BigQueryClient};
use std::{collections::HashMap, str::FromStr, sync::Arc, time::Duration};

const ANALYTICS_PROJECT_ID: &str = "analytics-test-345723";

#[derive(Clone, Debug)]
pub struct PeerLocation {
    pub peer_id: PeerId,
    pub country: Option<String>,
    pub region: Option<String>,
    pub geo_updated_at: Option<String>,
}

pub struct PeerLocationUpdater {
    client: BigQueryClient,
    peer_locations: Arc<RwLock<HashMap<PeerId, PeerLocation>>>,
}

impl PeerLocationUpdater {
    pub fn new(
        client: BigQueryClient,
        peer_locations: Arc<RwLock<HashMap<PeerId, PeerLocation>>>,
    ) -> Self {
        Self {
            client,
            peer_locations,
        }
    }

    pub fn run(self) -> anyhow::Result<()> {
        tokio::spawn(async move {
            loop {
                let locations = query_peer_locations(&self.client).await.unwrap();
                {
                    let mut peer_locations = self.peer_locations.write();
                    *peer_locations = locations;
                }
                tokio::time::sleep(Duration::from_secs(3600)).await; // 1 hour
            }
        });
        Ok(())
    }
}



pub async fn query_peer_locations(
    client: &BigQueryClient,
) -> anyhow::Result<HashMap<PeerId, PeerLocation>> {
    let req = QueryRequest::new("
        WITH latest_epochs AS (
            SELECT
                chain_id,
                MAX(epoch) AS max_epoch
            FROM `node-telemetry.aptos_data.validator_perf`
            GROUP BY chain_id
        ),
        latest_validator_pool AS (
            SELECT
                chain_id,
                CONCAT('0x', LPAD(LTRIM(peer_id, '0x'), 64, '0')) as peer_id
            FROM `node-telemetry.aptos_data.validator_perf`
            JOIN latest_epochs USING (chain_id)
            WHERE epoch = max_epoch
        ),
        latest_node_ips AS (
            SELECT DISTINCT
                n.chain_id,
                n.epoch,
                element.value as ip,
                CONCAT('0x', LPAD(LTRIM(n.peer_id, '0x'), 64, '0')) as peer_id
            FROM `analytics-test-345723.aptos_node_telemetry.custom_events` n
            JOIN latest_epochs le ON n.chain_id = CAST(le.chain_id AS STRING)
            CROSS JOIN UNNEST(event_params) as element
            WHERE
                element.key = 'IP_ADDRESS'
                AND n.epoch = le.max_epoch
        )
        SELECT
            a.peer_id,
            ip,
            ipg.country,
            ipg.region,
            ipg.update_timestamp
        FROM latest_node_ips a
        LEFT JOIN `node-telemetry.aptos_node_telemetry.ip_geo_latest` ipg
        USING(ip)
    ");
    let req = QueryRequest {
        timeout_ms: Some(10000),
        ..req
    };

    BIG_QUERY_REQUEST_TOTAL.inc();

    let mut res = client
        .job()
        .query(ANALYTICS_PROJECT_ID, req)
        .await
        .map_err(|e| {
            BIG_QUERY_REQUEST_FAILURES_TOTAL.inc();
            aptos_logger::error!("Failed to query peer locations: {}", e);
            e
        })?;

    let mut map = HashMap::new();
    while res.next_row() {
        if let Some(peer_id_raw) = res.get_string_by_name("peer_id")? {
            match PeerId::from_str(&peer_id_raw) {
                Ok(peer_id) => {
                    let location = PeerLocation {
                        peer_id,
                        geo_updated_at: res.get_string_by_name("update_timestamp")?,
                        country: res.get_string_by_name("country")?,
                        region: res.get_string_by_name("region")?,
                    };
                    map.entry(peer_id).or_insert(location);
                },
                Err(e) => {
                    aptos_logger::error!("Failed to parse peer_id: {}", e);
                },
            }
        }
    }
    Ok(map)
}
#[cfg(feature = "bigquery_integration_tests")]
mod tests {
    use super::*;
    use gcp_bigquery_client::Client as BigQueryClient;

    #[tokio::test]
    async fn test_query() {
        let client = BigQueryClient::from_application_default_credentials()
            .await
            .unwrap();
        let result = query_peer_locations(&client).await.unwrap();
        println!("{:?}", result);
        assert!(!result.is_empty());
    }
}
