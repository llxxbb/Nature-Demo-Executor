extern crate serde_json;

use nature_common::{ConverterParameter, ConverterReturned, generate_id, Instance};
use nature_demo_common::{Order, OrderAccount, OrderAccountReason, Payment};

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes)]
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
#[allow(improper_ctypes)]
pub extern fn pay_count(para: &ConverterParameter) -> ConverterReturned {
    let payment: Payment = serde_json::from_str(&para.from.content).unwrap();
    if para.last_state.is_none() {
        return ConverterReturned::EnvError;
    }
    let old = para.last_state.as_ref().unwrap();
    let mut oa: OrderAccount = serde_json::from_str(&old.content).unwrap();
    let mut state = String::new();
    if payment.paid > 0 {
        state = "partial".to_string();
    }
    oa.total_paid += payment.paid;
    oa.diff = oa.total_paid as i32 - oa.receivable as i32;
    if oa.diff > 0 {
        oa.total_paid = oa.receivable;
    }
    if oa.diff == 0 {
        state = "paid".to_string();
    }
    oa.last_paid = payment.paid;
    oa.reason = OrderAccountReason::Pay;
    let mut instance = Instance::default();
    instance.content = serde_json::to_string(&oa).unwrap();
    instance.states.insert(state);
    ConverterReturned::Instances(vec![instance])
}

#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes)]
pub extern fn go_express(para: &ConverterParameter) -> ConverterReturned {
    // "any one" will be correct by Nature after returned
    let mut ins = Instance::new("any one").unwrap();
    ins.context.insert("sys.target".to_owned(), para.from.id.to_string());
    // ... some code to  submit package info to the express company,
    // ... and wait it to return an id.
    // the follow line simulate the express company name and the waybill id returned
    ins.para = "/ems/".to_owned() + &generate_id(&para.master.clone().unwrap().data).unwrap().to_string();
    // return the waybill
    ConverterReturned::Instances(vec![ins])
}