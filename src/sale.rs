use nature_common::{ConverterParameter, ConverterReturned, Instance, Result};
use nature_demo_common::Order;

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes)]
pub extern fn item_statistics(para: &ConverterParameter) -> ConverterReturned {
    dbg!(&para.from.content);
    ConverterReturned::Instances(vec![])
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes)]
pub extern fn order2item(para: &Instance) -> Result<Instance> {
    let order: Order = serde_json::from_str(&para.content)?;
    let mut content: Vec<(String, u64)> = vec![];
    for one in order.items {
        let id = one.item.id;
        let money_key = id.to_string() + &"/money".to_string();
        let count_key = id.to_string() + &"/count".to_string();
        content.push((money_key, one.num as u64 * one.item.price));
        content.push((count_key, one.num as u64));
    }
    let mut rtn = para.clone();
    rtn.content = serde_json::to_string(&content)?;
    Ok(rtn)
}