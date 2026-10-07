use holo_protocol::{
    InstanceChannelsTx, InstanceShared, MessageReceiver, ProtocolInstance,
};
use holo_utils::ibus::IbusMsg;
use holo_utils::protocol::Protocol;
use holo_yang::ToYang;

use crate::northbound::{configuration, rpc, state};

#[derive(Debug)]
pub struct Instance {
    pub name: String,
    pub tx: InstanceChannelsTx<Instance>,
    pub shared: InstanceShared,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ProtocolInputMsg {}

#[derive(Debug, serde::Serialize)]
pub enum ProtocolOutputMsg {}

#[derive(Clone, Debug)]
pub struct ProtocolInputChannelsTx;

#[derive(Debug)]
pub struct ProtocolInputChannelsRx;

impl ProtocolInstance for Instance {
    const PROTOCOL: Protocol = Protocol::PIM;

    type ProtocolInputMsg = ProtocolInputMsg;
    type ProtocolOutputMsg = ProtocolOutputMsg;
    type ProtocolInputChannelsTx = ProtocolInputChannelsTx;
    type ProtocolInputChannelsRx = ProtocolInputChannelsRx;

    fn new(
        name: String,
        shared: InstanceShared,
        tx: InstanceChannelsTx<Instance>,
    ) -> Self {
        Self { name, tx, shared }
    }

    fn process_ibus_msg(&mut self, _msg: IbusMsg) {}

    fn process_protocol_msg(&mut self, _msg: ProtocolInputMsg) {}

    fn protocol_input_channels() -> (
        ProtocolInputChannelsTx,
        ProtocolInputChannelsRx,
    ) {
        (ProtocolInputChannelsTx, ProtocolInputChannelsRx)
    }
}

impl MessageReceiver<ProtocolInputMsg> for ProtocolInputChannelsRx {
    async fn recv(&mut self) -> Option<ProtocolInputMsg> {
        None
    }
}

impl holo_northbound::configuration::Provider for Instance {
    type Event = ();
    type Resource = ();
    type Change = configuration::Change;

    const YANG_OPS_CONFIG: holo_northbound::configuration::YangConfigOps<
        Self::Change,
    > = configuration::YANG_OPS_CONFIG;
}

impl holo_northbound::rpc::Provider for Instance {
    const YANG_OPS: holo_northbound::rpc::YangOps<Self> = rpc::YANG_OPS;
}

impl holo_northbound::state::Provider for Instance {
    type ListEntry<'a> = state::ListEntry;

    const YANG_OPS: holo_northbound::state::YangOps<Self> = state::YANG_OPS;

    fn top_level_node(&self) -> String {
        format!(
            "/ietf-routing:routing/control-plane-protocols/control-plane-protocol[type='{}'][name='{}']/holo-pim:pim",
            Protocol::PIM.to_yang(),
            self.name,
        )
    }
}