use nature_common::{ConverterParameter, ConverterReturned, Instance};

#[no_mangle]
#[allow(unused_attributes)]
pub extern fn order_new(_para: &ConverterParameter) -> ConverterReturned {
    let mut instance = Instance::default();
    instance.data.content = "".to_string();
    ConverterReturned::Instances(vec![instance])
}

#[no_mangle]
#[allow(unused_attributes)]
pub extern fn order_receivable(_para: &ConverterParameter) -> ConverterReturned {
    let mut instance = Instance::default();
    instance.data.content = "".to_string();
    ConverterReturned::Instances(vec![instance])
}

#[no_mangle]
#[allow(unused_attributes)]
pub extern fn order_paid(_para: &ConverterParameter) -> ConverterReturned {
    let mut instance = Instance::default();
    instance.data.content = "".to_string();
    ConverterReturned::Instances(vec![instance])
}