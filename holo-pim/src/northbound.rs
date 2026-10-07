pub mod configuration {
    pub type Change = ();

    pub const YANG_OPS_CONFIG: holo_northbound::configuration::YangConfigOps<
        Change,
    > = holo_northbound::configuration::YangConfigOps {
        parse: phf::phf_map! {},
        change_keys: &[],
    };
}

pub mod rpc {
    pub const YANG_OPS: holo_northbound::rpc::YangOps<crate::instance::Instance> =
        holo_northbound::rpc::YangOps {
            rpc: phf::phf_map! {},
        };
}

pub mod state {
    #[derive(Debug, Default)]
    pub struct ListEntry;

    impl holo_northbound::state::ListEntryKind for ListEntry {}

    pub const YANG_OPS: holo_northbound::state::YangOps<crate::instance::Instance> =
        holo_northbound::state::YangOps {
            list: phf::phf_map! {},
            container: phf::phf_map! {},
        };
}