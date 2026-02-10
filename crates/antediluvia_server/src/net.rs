//! Networking bootstrap for the authoritative server.
//! Uses bevy_renet config placeholders; fill in real transport later.

use antediluvia_core::{NetworkConfig, NetworkMessage, PlayerNetworkState};
use tracing::info;
use anyhow::Result;
use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::Duration;
use bevy_renet::renet::{
    RenetServer, ConnectionConfig, ServerEvent,
};
use bevy_renet::netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bincode;

/// Lightweight placeholder server state until renet transport is wired.
#[derive(Default)]
pub struct NetServer {
    pub player_states: HashMap<u64, PlayerNetworkState>,
    pub handles: Option<NetHandles>,
}

/// Handles to the running transport/server.
pub struct NetHandles {
    pub server: RenetServer,
    pub transport: NetcodeServerTransport,
}

impl NetServer {
    pub fn new() -> Self {
        Self { player_states: HashMap::new(), handles: None }
    }

    /// Start the networking layer.
    pub fn start_server(&mut self, config: &NetworkConfig) -> Result<()> {
        info!("(net) initializing server at {}:{}", config.server_addr, config.server_port);

        // Channel config defaults
        let connection_config = ConnectionConfig::default();
        let server = RenetServer::new(connection_config);

        // Netcode config
        let server_config = ServerConfig {
            current_time: Duration::from_secs(0),
            max_clients: config.max_players as usize,
            protocol_id: 7,
            authentication: ServerAuthentication::Unsecure,
            public_addresses: vec![format!("{}:{}", config.server_addr, config.server_port).parse()?],
        };

        let socket = UdpSocket::bind(format!("{}:{}", config.server_addr, config.server_port))?;
        socket.set_nonblocking(true)?;
        let transport = NetcodeServerTransport::new(server_config, socket)?;

        self.handles = Some(NetHandles { server, transport });
        Ok(())
    }

    /// Pump the transport/server (no-op if not started).
    pub fn update(&mut self, delta: Duration) -> Result<()> {
        if let Some(handles) = &mut self.handles {
            handles.server.update(delta);
            handles.transport.update(delta, &mut handles.server)?;
        }
        Ok(())
    }

    /// Send queued packets after update.
    pub fn send_packets(&mut self) {
        if let Some(handles) = &mut self.handles {
            handles.transport.send_packets(&mut handles.server);
        }
    }

    /// Poll server events (connect/disconnect).
    pub fn poll_events(&mut self) -> Vec<ServerEvent> {
        let mut events = Vec::new();
        if let Some(handles) = &mut self.handles {
            while let Some(event) = handles.server.get_event() {
                events.push(event);
            }
        }
        events
    }
}

impl NetServer {
    /// Receive all pending messages from clients (channel 0).
    pub fn receive_messages(&mut self) -> Vec<(u64, NetworkMessage)> {
        let mut out = Vec::new();
        if let Some(handles) = &mut self.handles {
            for client_id in handles.server.clients_id().into_iter() {
                while let Some(raw) = handles.server.receive_message(client_id, 0) {
                    if let Ok(msg) = bincode::deserialize::<NetworkMessage>(&raw) {
                        out.push((client_id, msg));
                    }
                }
            }
        }
        out
    }

    /// Broadcast a message to all clients (channel 0).
    pub fn broadcast(&mut self, msg: &NetworkMessage) -> Result<()> {
        if let Some(handles) = &mut self.handles {
            let payload = bincode::serialize(msg)?;
            for client_id in handles.server.clients_id().into_iter() {
                handles.server.send_message(client_id, 0, payload.clone());
            }
        }
        Ok(())
    }

    /// Disconnect a client.
    pub fn disconnect(&mut self, client_id: u64) {
        if let Some(handles) = &mut self.handles {
            handles.server.disconnect(client_id);
        }
    }
}
