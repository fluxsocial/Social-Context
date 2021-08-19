import { Config, InstallAgentsHapps } from '@holochain/tryorama'
import { TransportConfigType, ProxyAcceptConfig, ProxyConfigType, NetworkType } from '@holochain/tryorama'
import path from 'path'

// Set up a Conductor configuration using the handy `Conductor.config` helper.
// Read the docs for more on configuration.
export const localConductorConfig = Config.gen();

// Set up a Conductor configuration using the handy `Conductor.config` helper.
// Read the docs for more on configuration.
export const network = {
    network_type: NetworkType.QuicBootstrap,
    transport_pool: [{
      type: TransportConfigType.Proxy,
      sub_transport: {type: TransportConfigType.Quic},
      proxy_config: {
        type: ProxyConfigType.LocalProxyServer,
        proxy_accept_config: ProxyAcceptConfig.AcceptAll
      }
    }],
    bootstrap_service: "https://bootstrap.holo.host",
    tuning_params: {
      gossip_loop_iteration_delay_ms: 10,
      default_notify_remote_agent_count: 5,
      default_notify_timeout_ms: 1000,
      default_rpc_single_timeout_ms:  2000,
      default_rpc_multi_remote_agent_count: 2,
      default_rpc_multi_timeout_ms: 2000,
      agent_info_expires_after_ms: 1000 * 60 * 20,
      tls_in_mem_session_storage: 512,
      proxy_keepalive_ms: 1000 * 60 * 2,
      proxy_to_expire_ms: 1000 * 6 * 5
    }
  }
  
  // create an InstallAgentsHapps array with your DNAs to tell tryorama what
  // to install into the conductor.
  export const installation: InstallAgentsHapps = [
    // agent 0
    [
      // happ 0
      [path.join("../../workdir/social-context.dna")]
    ]
  ]
  
  export const sleep = ms => new Promise(r => setTimeout(r, ms))
