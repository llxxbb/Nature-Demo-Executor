use nature_common::{CallOutParameter, ConverterReturned, Instance};

#[no_mangle]
#[allow(unused_attributes)]
pub extern fn order_new(_para: &CallOutParameter) -> ConverterReturned {
    let mut instance = Instance::default();
    instance.data.content = "".to_string();
    ConverterReturned::Instances(vec![instance])
}
