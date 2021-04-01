use super::candidate_base::*;
use super::*;
use crate::errors::*;
use crate::rand::generate_cand_id;
use crate::util::*;
use std::sync::atomic::{AtomicU16, AtomicU8};
use std::sync::Arc;

// CandidatePeerReflexiveConfig is the config required to create a new CandidatePeerReflexive
#[derive(Default)]
pub struct CandidatePeerReflexiveConfig {
    pub base_config: CandidateBaseConfig,

    pub rel_addr: String,
    pub rel_port: u16,
}

impl CandidatePeerReflexiveConfig {
    // new_candidate_peer_reflexive creates a new peer reflective candidate
    pub async fn new_candidate_peer_reflexive(
        self,
        agent_internal: Option<Arc<Mutex<AgentInternal>>>,
    ) -> Result<CandidateBase, Error> {
        let ip: IpAddr = match self.base_config.address.parse() {
            Ok(ip) => ip,
            Err(_) => return Err(ERR_ADDRESS_PARSE_FAILED.to_owned()),
        };
        let network_type = determine_network_type(&self.base_config.network, &ip)?;

        let mut candidate_id = self.base_config.candidate_id;
        if candidate_id.is_empty() {
            candidate_id = generate_cand_id();
        }

        let c = CandidateBase {
            id: candidate_id,
            network_type: Arc::new(AtomicU8::new(network_type as u8)),
            candidate_type: CandidateType::PeerReflexive,
            address: self.base_config.address,
            port: self.base_config.port,
            resolved_addr: Arc::new(Mutex::new(create_addr(
                network_type,
                ip,
                self.base_config.port,
            ))),
            component: Arc::new(AtomicU16::new(self.base_config.component)),
            foundation_override: self.base_config.foundation,
            priority_override: self.base_config.priority,
            related_address: Some(CandidateRelatedAddress {
                address: self.rel_addr,
                port: self.rel_port,
            }),
            conn: self.base_config.conn,
            agent_internal,
            ..Default::default()
        };

        Ok(c)
    }
}
