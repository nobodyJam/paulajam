
extern crate http_req;
extern crate encoding;
extern crate winapi;
extern crate json;
extern crate libc;

/// stdcall形式的一个动态库
/// 提供
/// 基础库 pjoo
/// JSON数据解析   pj_json
/// HTTP 客户端访问 pj_http
/// 字符编码转换 pj_encoding
/// ```
/// assert_eq!(2 + 2, 4);
/// 
/// ```
/// 
/// 
/// 
pub mod pjoo;
pub mod pbwinapi;
pub mod pj_json;
pub mod pj_http;

use crate::pjoo::*;

/* pjoo 中的导出函数 */
#[export_name="free_cstring"] pub extern "stdcall" fn free_cstring(s:PbRData){ pjoo::free_cstring(s);}
#[export_name="str_ptr"] pub extern "stdcall" fn str_ptr(s:PbReadonlyString)->PbReadonlyString{pjoo::str_ptr(s)}

/* pbwinapi 中的导出函数 */
#[export_name="run_cmd"]pub extern "stdcall" fn run_cmd(cmd:PbReadonlyString,fgf:PbReadonlyString)->PbRefString{
    pbwinapi::run_cmd(cmd,fgf)
}
#[export_name="run_cmd_hide"]pub extern "stdcall" fn  run_cmd_hide(a_s:PbRefString)->PbReadonlyString{
    pbwinapi::run_cmd_hide(a_s)
}
/* pj_json 中的导出函数 */

/* pj_http 中的导出函数 */

