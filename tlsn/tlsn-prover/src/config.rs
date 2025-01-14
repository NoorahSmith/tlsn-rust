use actor_ot::{OTActorReceiverConfig, OTActorSenderConfig};
use mpc_share_conversion::{ReceiverConfig, SenderConfig};
use tls_client::RootCertStore;
use tls_mpc::{MpcTlsCommonConfig, MpcTlsLeaderConfig};

const DEFAULT_MAX_TRANSCRIPT_SIZE: usize = 1 << 14; // 16Kb

#[derive(Debug, Clone, derive_builder::Builder)]
pub struct ProverConfig {
    /// Id of the notarization session.
    #[builder(setter(into))]
    id: String,
    /// The server DNS name.
    #[builder(setter(into))]
    server_dns: String,
    /// TLS root certificate store.
    #[builder(setter(strip_option), default = "default_root_store()")]
    pub(crate) root_cert_store: RootCertStore,
    /// Maximum transcript size in bytes
    ///
    /// This includes the number of bytes sent and received to the server.
    #[builder(default = "DEFAULT_MAX_TRANSCRIPT_SIZE")]
    max_transcript_size: usize,
}

impl ProverConfig {
    /// Create a new builder for `ProverConfig`.
    pub fn builder() -> ProverConfigBuilder {
        ProverConfigBuilder::default()
    }

    /// Get the maximum transcript size in bytes.
    pub fn max_transcript_size(&self) -> usize {
        self.max_transcript_size
    }

    /// Returns the server DNS name.
    pub fn server_dns(&self) -> &str {
        &self.server_dns
    }

    pub(crate) fn build_mpc_tls_config(&self) -> MpcTlsLeaderConfig {
        MpcTlsLeaderConfig::builder()
            .common(
                MpcTlsCommonConfig::builder()
                    .id(format!("{}/mpc_tls", &self.id))
                    .handshake_commit(true)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
    }

    pub(crate) fn build_ot_sender_config(&self) -> OTActorSenderConfig {
        OTActorSenderConfig::builder()
            .id("ot/0")
            .initial_count(self.max_transcript_size() * 8)
            .build()
            .unwrap()
    }

    pub(crate) fn build_ot_receiver_config(&self) -> OTActorReceiverConfig {
        OTActorReceiverConfig::builder()
            .id("ot/1")
            .initial_count(self.max_transcript_size() * 8)
            .committed()
            .build()
            .unwrap()
    }

    pub(crate) fn build_p256_sender_config(&self) -> SenderConfig {
        SenderConfig::builder().id("p256/0").build().unwrap()
    }

    pub(crate) fn build_p256_receiver_config(&self) -> ReceiverConfig {
        ReceiverConfig::builder().id("p256/1").build().unwrap()
    }

    pub(crate) fn build_gf2_config(&self) -> SenderConfig {
        SenderConfig::builder().id("gf2").record().build().unwrap()
    }
}

/// Default root store using mozilla certs.
fn default_root_store() -> RootCertStore {
    let mut root_store = tls_client::RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        tls_client::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));
    root_store
}
