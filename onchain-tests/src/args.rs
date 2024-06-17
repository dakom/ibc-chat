use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = process)]
    type Process;
    #[wasm_bindgen(js_name = process)]
    static PROCESS: Process;
    #[wasm_bindgen(method, getter)]
    fn argv(this: &Process) -> Vec<String>;
}

pub fn arg_var(key: &str) -> Option<String> {

    let args = PROCESS.argv();

    args.into_iter().find_map(|arg| {
        let mut split = arg.split("=");
        match (split.next(), split.next()) {
            (Some(k), Some(v)) => {
                if k == &format!("--{}", key) {
                    Some(v.to_string())
                } else {
                    None
                }
            },
            _ => None
        }
    })
}
