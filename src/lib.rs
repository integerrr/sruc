pub mod ei_struct;

pub mod ei {
    include!(concat!(env!("OUT_DIR"), "/ei.rs"));
}

pub mod table_builder;

pub mod ei_request;
