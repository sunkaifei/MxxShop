use rbatis::{Page, PageRequest};
use crate::modules::system::entity::ip_address_entity::IpAddress;
use crate::modules::system::entity::ip_address_model::{IpAddressPageDTO, QueryPageRequest};
use crate::modules::system::mapper::ip_address_mapper;
use crate::RB;

///添加ip地址
/// 目前逻辑有问题
pub async fn ip_address_save(mut item: &IpAddress) -> rbatis::Result<u64> {

    let ip_list = ip_address_mapper::select_ip_address_list(&RB.clone(), &item.start_ip, &item.end_ip).await?;
    for ip_vo in &ip_list {
        if ip_list.clone().len() == 1 {
            //当查询出是一条数据时，该IP数据段被分为两端，需要进行分割数据，修改当前数据，并增加新的数据段
            let update_ip = IpAddress {
                id: ip_vo.id,
                start_ip: None,
                end_ip: Some(item.end_ip.unwrap_or_default() - 1),
                country: None,
                province: None,
                city: None,
                county: None,
                address: None,
                local: None,
            };
            ip_address_update(&update_ip).await?;
            
            let add_ip = IpAddress {
                id: ip_vo.id,
                start_ip: Some(item.end_ip.unwrap_or_default()+1),
                end_ip: ip_vo.end_ip,
                country: item.country.clone(),
                province: item.province.clone(),
                city: item.city.clone(),
                county: item.county.clone(),
                address: item.address.clone(),
                local: item.local.clone(),
            };
            IpAddress::insert(&RB.clone(), &add_ip).await?;
        }else {
            if ip_vo.start_ip > item.start_ip && ip_vo.end_ip < item.end_ip {
                IpAddress::delete_by_column(&RB.clone(), "id", &ip_vo.id).await?;
            }
        }
        
    }
    let res = IpAddress::insert(&RB.clone(), &item).await?;
    return Ok(u64::from(res.last_insert_id));
}

///按id删除ip地址
pub async fn delete_ip_address(id: Option<u64>) -> rbatis::Result<u64> {
    let res = IpAddress::delete_by_column(&RB.clone(), "id", id).await?;
    return Ok(u64::from(res.rows_affected));
}

///批量删除ip地址
pub async fn bath_delete_ip_address(ids: Vec<Option<String>>) -> rbatis::Result<u64> {
    let res = IpAddress::delete_in_column(&RB.clone(), "id", &ids).await?;
    return Ok(u64::from(res.rows_affected));
}

///更新ip地址
pub async fn ip_address_update(item: &IpAddress) -> rbatis::Result<u64> {
    let res = IpAddress::update_by_column(&RB.clone(), &item,"id").await?;
    return Ok(u64::from(res.rows_affected));
}



///按id查询ip地址
pub async fn get_ip_address_by_id(id: u64) -> rbatis::Result<Option<IpAddress>> {
    let st = IpAddress::select_by_column(&RB.clone(), "id", id).await?
        .into_iter()
        .next();
    return Ok(st);
}

///按ip查询ip地址
pub async fn find_ip_address_by_ip(ip: &Option<u32>) -> rbatis::Result<Option<IpAddress>> {
    let st = ip_address_mapper::find_ip_address_by_ip(&RB.clone(), ip).await?.into_iter().next();
    return Ok(st);
}

///获取ip地址列表翻页
pub async fn ip_address_page(item: QueryPageRequest) -> rbatis::Result<Page<IpAddress>> {
    let ip_dto:IpAddressPageDTO = item.into();
    let page_req = &PageRequest::new_option(ip_dto.page_num.clone(), ip_dto.page_size.clone());
    let list = IpAddress::select_page(&RB.clone(), page_req, &ip_dto).await?;
    return Ok(list);
}