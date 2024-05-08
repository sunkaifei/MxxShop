use std::net::Ipv4Addr;
use actix_web::{get, HttpResponse};
use actix_web::web::Query;
use crate::core::web::response::{ok_result_page, ResultPage, ResVO};
use crate::modules::system::entity::ip_address_model::{IpListVO, QueryPageRequest};
use crate::modules::system::service::ip_address_servive;

#[get("/system/ip/list")]
pub async fn ip_address_page(item: Query<QueryPageRequest>) -> HttpResponse {
    let ip_request = item.0;
    let page_result = ip_address_servive::ip_address_page(ip_request).await;
    match page_result {
        Ok(page) => {
            let mut list_data: Vec<IpListVO> = Vec::new();
            for ip in page.records {
                let ip_start = Ipv4Addr::from(ip.start_ip.unwrap_or_default());
                let ip_end = Ipv4Addr::from(ip.end_ip.unwrap_or_default());
                list_data.push(IpListVO {
                    id: ip.id,
                    start_ip: Option::from(ip_start.to_string()),
                    end_ip: Option::from(ip_end.to_string()),
                    country: ip.country,
                    province: ip.province,
                    city: ip.city,
                    county: ip.county,
                    address: ip.address,
                    local: ip.local,
                })
            }
            let page_data = ResultPage {
                current_page: page.page_no,
                list: list_data,
                total: page.total,
            };
            HttpResponse::Ok().json(ok_result_page(page_data))
        }
        Err(err) => {
            HttpResponse::Ok().json(ResVO::<String>::error_msg(
                "id地址列表异常,".to_string() + &err.to_string()
            ))
        }
    }
    
}