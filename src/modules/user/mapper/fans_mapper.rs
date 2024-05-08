use rbatis::{RBatis, sql};


/// 根据被关注id和粉丝查询是否已关注
#[sql("select count(*) from fly_user_fans where follow_id = ? and fans_id = ?")]
pub async fn find_by_fans_unique(
    rb: &RBatis,
    follow_id: &Option<u64>,
    fans_id: &Option<u64>,
) -> rbatis::Result<u64> {
    impled!()
}