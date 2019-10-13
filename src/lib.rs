extern crate serde_json;

use nature_common::{ConverterParameter, ConverterReturned, Instance};
use nature_demo_common::{Order, OrderAccount, OrderAccountReason, Payment};

#[no_mangle]
#[allow(unused_attributes)]
pub extern fn order_receivable(para: &ConverterParameter) -> ConverterReturned {
    let order: Order = serde_json::from_str(&para.from.content).unwrap();
    let oa = OrderAccount {
        receivable: order.price,
        total_paid: 0,
        last_paid: 0,
        reason: OrderAccountReason::NewOrder,
        diff: 0 - order.price as i32,
    };
    let mut instance = Instance::default();
    instance.content = serde_json::to_string(&oa).unwrap();
    ConverterReturned::Instances(vec![instance])
}

#[no_mangle]
#[allow(unused_attributes)]
pub extern fn pay_count(para: &ConverterParameter) -> ConverterReturned {
    dbg!(&para.from.content);
    let payment: Payment = serde_json::from_str(&para.from.content).unwrap();
    dbg!(&para.last_state);
    let old = para.last_state.as_ref().unwrap();
    dbg!(&old.content);
    let mut oa: OrderAccount = serde_json::from_str(&old.content).unwrap();
    oa.total_paid += payment.paid;
    oa.diff = oa.total_paid as i32 - oa.receivable as i32;
    if oa.diff > 0 {
        oa.total_paid = oa.receivable;
    }
    oa.last_paid = payment.paid;
    oa.reason = OrderAccountReason::Pay;
    let mut instance = Instance::default();
    instance.content = serde_json::to_string(&oa).unwrap();
    ConverterReturned::Instances(vec![instance])
}
