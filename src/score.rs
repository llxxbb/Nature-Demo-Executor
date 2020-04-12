use nature_common::{Instance, NatureError, Result};


#[no_mangle]
#[allow(unused_attributes)]
#[allow(improper_ctypes)]
pub extern fn person_score_filter(para: &Vec<Instance>) -> Result<Vec<Instance>> {
    let mut rtn: Vec<Instance> = vec![];
    for input in para {
        let mut one = input.clone();
        let kv = serde_json::from_str::<Vec<KV>>(&input.content);
        let kv = match kv {
            Ok(kv) => kv,
            Err(err) => {
                return Err(NatureError::LogicalError(err.to_string()));
            }
        };
        if kv.len() != 1 {
            return Err(NatureError::LogicalError("should return one item".to_owned()));
        }
        one.content = kv[0].value.to_string();
        rtn.push(one);
    }
    Ok(rtn)
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct KV {
    #[allow(dead_code)]
    pub key: String,
    pub value: i32,
}
