use std::time::Duration;
use log::LevelFilter;
use rbatis::intercept_log::LogInterceptor;
use crate::RB;
use crate::utils::settings::Settings;

// 连接数据库
pub async fn init_db() {
    let setting = Settings::get();
    RB.init(rbdc_mysql::driver::MysqlDriver {}, &setting.database.mysql_url, ).unwrap();
    RB.get_pool().unwrap().set_max_open_conns(setting.database.db_pool_len as u64).await;
    RB.get_pool().unwrap().set_timeout(Some(Duration::from_secs(setting.database.db_pool_timeout as u64))).await;
    //日志开关或者等级设置
    // off：关闭日志
    //
    // Error：用于记录错误信息，表示发生了错误或异常情况。这种日志级别通常用于记录程序无法正常执行的情况，并可能导致程序中断或异常退出。
    // Warn：用于记录警告信息，表示可能存在潜在问题或错误的情况。这些信息不会导致程序中断，但可能需要开发者的关注。
    // Info：用于记录普通的信息日志，通常用于表示程序的状态、进程的进展等一般性的信息。
    // Debug：用于记录调试信息，比 trace!() 级别稍微高一级。它通常用于输出一些有用的调试信息，但比 trace!() 更为精简。
    // Trace：用于记录非常详细的调试信息，通常在最低级别使用。这些日志信息可以帮助您跟踪代码的执行路径、变量的值等详细信息。
    //RB.get_intercept::<LogInterceptor>().unwrap().set_level_filter(LevelFilter::Off);
    log::info!(
        "[fly_cms] 数据库连接成功！ = {}",
        RB.get_pool().expect("数据连接失败!").state().await
    );
}
