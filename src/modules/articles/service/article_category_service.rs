use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;

use crate::modules::articles::entity::article_category_entity::ArticleCategory;
use crate::modules::articles::entity::article_category_model::{ArticleCategorySaveRequest, CategoryPageRequest, CategoryTree, CategoryTreeVO};
use crate::modules::articles::mapper::article_category_mapper;
use crate::RB;
use crate::utils::{short_url, snowflake_id};

///添加菜单
pub async fn save_category(payload: ArticleCategorySaveRequest) -> rbatis::Result<ExecResult> {
    let mut category_data: ArticleCategory = payload.into();
    match snowflake_id::generate_unique_id() {
        Ok(id) => {
            category_data.id = id;
        }
        Err(err) => {
            return Err(Error::from(err.to_string()));
        }
    }

    let unique_num = article_category_mapper::find_by_category_name_unique(&RB.clone(), &category_data.category_name).await?;
    if unique_num > 0 {
        return Err(Error::from("栏目名称已存在".to_string()));
    }

    //获取短网址唯一性
    category_data.short_url = find_short_url_unique().await;

    return ArticleCategory::insert(&RB.clone(), &category_data).await;
}


pub async fn find_short_url_unique() -> Option<String> {
    let mut new_short_url: Option<String> = Some(String::new());
    for _ in 0..5 {
        //获取短网址
        let short_url = short_url::generate_random_code(5).unwrap_or_default();
        let unique_num = article_category_mapper::find_by_short_url_unique(&RB.clone(), &short_url).await;
        if unique_num.unwrap_or(0) == 0 {
            new_short_url = Some(short_url);
            break;
        }
    }
    return new_short_url;
}

///获取所有菜单列表树
pub async fn all_ategory_tree() -> rbatis::Result<Vec<CategoryTree>> {
    let list: Vec<ArticleCategory> = ArticleCategory::select_all(&mut RB.clone()).await?;
    let mut router_list = Vec::<CategoryTree>::new();
    ategory_arr_to_tree(&mut router_list, list, 0);
    Ok(router_list)
}

// 菜单数组转树
pub fn ategory_arr_to_tree(re_list: &mut Vec<CategoryTree>, ori_arr: Vec<ArticleCategory>, pid: u64) {
    for (_, it) in ori_arr.iter().enumerate() {
        if pid == it.parent_id {
            let mut children = Vec::<CategoryTree>::new();
            ategory_arr_to_tree(&mut children, ori_arr.clone(), it.id);
            let temp_router = CategoryTree {
                children: (|| -> Option<Vec<CategoryTree>> {
                    if children.len() > 0 {
                        Some(children)
                    } else {
                        None
                    }
                })(),
                id: it.id.clone(),
                label: it.category_name.clone(),
            };
            re_list.push(temp_router)
        }
    }
}

// 菜单数组转树
pub fn ategory_list_arr_to_tree(re_list: &mut Vec<CategoryTreeVO>, ori_arr: Vec<ArticleCategory>, pid: u64) {
    for (_, it) in ori_arr.iter().enumerate() {
        if pid == it.parent_id {
            let mut children = Vec::<CategoryTreeVO>::new();
            ategory_list_arr_to_tree(&mut children, ori_arr.clone(), it.id);
            let temp_router = CategoryTreeVO {
                id: it.id.clone(),
                parent_id: it.parent_id.clone(),
                short_url: None,
                user_id: None,
                category_name: it.category_name.clone(),
                sort: 0,
                count_topic: 0,
                create_time: None,
                update_time: None,
                is_show: 0,
                status: it.status.clone(),
                children: (|| -> Option<Vec<CategoryTreeVO>> {
                      if children.len() > 0 {
                        Some(children)
                         } else {
                            None
                      }
                    })(),
            };
            re_list.push(temp_router)
        }
    }
}

// 查询部门所有数据列表
pub async fn select_all_list(item: CategoryPageRequest) -> rbatis::Result<Vec<CategoryTreeVO>> {
    let list: Vec<ArticleCategory> = ArticleCategory::select_all(&mut RB.clone()).await?;
    let mut dept_list = Vec::<CategoryTreeVO>::new();
    ategory_list_arr_to_tree(&mut dept_list, list, 0);
    Ok(dept_list)
}
