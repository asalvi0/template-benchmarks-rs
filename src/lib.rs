pub mod askama;
pub mod fomat;
pub mod handlebars;
pub mod horrorshow;
pub mod liquid;
pub mod markup;
pub mod maud;
pub mod minijinja;
pub mod ramhorns;
pub mod ructe;
pub mod sailfish;
pub mod std_write;
pub mod tera;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
