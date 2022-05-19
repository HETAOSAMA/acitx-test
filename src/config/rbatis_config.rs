use log::LevelFilter;
use rbatis::core::db::DBPoolOptions;

use rbatis::plugin::intercept::SqlIntercept;
use rbatis::plugin::log::LogPlugin;


use std::time::Duration;

use lazy_static::lazy_static;
use rbatis::rbatis::Rbatis;
//



lazy_static! {
        pub static ref rb : Rbatis = Rbatis::new();
}


pub struct InitDb;

impl InitDb {

    pub fn db_option() -> DBPoolOptions {
        let mut opt = DBPoolOptions::new();
        opt.min_connections = 1;
        opt.max_connections = 100;
        opt.connect_timeout = Duration::from_secs(100);
        opt.max_lifetime = Some(Duration::from_secs(1800));
        opt.test_before_acquire = true;
        opt
    }
}

