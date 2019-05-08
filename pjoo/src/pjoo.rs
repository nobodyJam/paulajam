
use std::ffi::{CStr,CString};
use libc::{c_char,c_void};
use encoding::{ DecoderTrap,EncoderTrap};
use encoding::label::encoding_from_whatwg_label;
use std::any::Any;

pub type PbC = * const c_char;


pub type PbReadonlyString =* const c_char;
pub type PbRefString =* mut c_char;
pub type PbReadonlyBlob =* const c_void;
pub type PbRefBlob =* mut c_void;
pub type PbRData =* mut c_void; 

/// 在pb中创建的字符串传入进来，不需要在rust中释放
/// 
/// 在rust中返回的字符串，在pb中取到值转换成pb中的字符串后需要再调用一个函数释放内存释放内存
/// 
/// PbValue::StringReadonly pb中传入readonly String
/// PbValue::BlobReadonly  pb中传入readonly Blob
/// PbValue::String pb中传入ref String
/// PbValue::Blob pb中传入ref blob
#[derive(Debug)]
pub enum PbValue {
    Null,
    StringReadonly(PbReadonlyString),
    BlobReadonly(PbReadonlyBlob),
    String(PbRefString),
    Blob(PbRefBlob),
    RData(* mut Box<Any>),
}

impl Drop for PbValue{
    fn drop(&mut self) {
        match self{
            PbValue::Blob(_)=>{},
            PbValue::String(_)=>{},
            PbValue::BlobReadonly(_)=>{},
            PbValue::StringReadonly(_)=>{},
            PbValue::RData(b)=>{unsafe{println!("BoxValue:{:?}Dropping!",*b);let _ = Box::from_raw(*b as * mut c_char);}},
            PbValue::Null=>{},
        }
        println!("PbValue Dropping!");
    }
}

impl PbValue{

    pub fn from_string(a: String)->Self{
        let ba = Box::into_raw(Box::new(Box::new(a) as Box<Any> ));
        PbValue::RData(ba)
    }
    pub fn from_vec(a: Vec<u8>)->Self{
        let ba = Box::into_raw(Box::new(Box::new(a) as Box<Any> ));
        PbValue::RData(ba)
    }

    pub unsafe fn from_rdata_raw(p:& mut PbRData)->Self{
        let pp = *p;
        *p = std::ptr::null_mut();
        if pp.is_null() { return PbValue::Null;}
        let b =  &mut *(pp as *mut Box<Any>);//如果p不是 *mut Box<Any> 会报错
        let c: Option<&mut String> = b.downcast_mut();
        if c.is_some() {
            PbValue::RData(pp as *mut Box<Any>)
        }else{
            let c: Option<&mut Vec<u8>> = b.downcast_mut();
            if c.is_some() {
                PbValue::RData(pp as *mut Box<Any>)
            }else{
                PbValue::Blob(pp)
            }
        }
    }


    /// 获取指针 会移动并销毁当前值 
    pub fn into_raw(self)->PbRData{
        match self{
            PbValue::Blob(b)=>{b as * mut c_void},
            PbValue::String(b)=>{b as * mut c_void},
            PbValue::BlobReadonly(b)=>{b as * mut c_void},
            PbValue::StringReadonly(b)=>{b as * mut c_void},
            PbValue::RData(b)=>{
                std::mem::forget(self);
                unsafe{
                    let bb = Box::from_raw(b);
                    Box::into_raw(bb) as * mut c_void
                }
            },
            PbValue::Null=>{std::ptr::null_mut()},
        }

        
    }
}


///
/// pb传过来的数据指针
pub trait Pb {
    /// 转换成str
    fn to_str<'a>(self)->&'a str;
    fn from_gbk_vec(d:Vec<u8>)->Self;
}

impl Pb for PbC{
    fn to_str<'a>(self)->&'a str{
        let bb =unsafe{ CStr::from_ptr(self) };
        bb.to_str().expect("转换成str错误!")
    }
    fn from_gbk_vec(d:Vec<u8>)->Self{
        d.as_ptr() as Self
    }
}

/// 将指定编码的数据转换成utf8
/// 如果转换错误将返回空字符串
pub fn to_utf8(data:&[u8],a:&str)->Option<String>{
    let t = encoding_from_whatwg_label(a);
    if t.is_none() {
        return None
    }
    let s = t.unwrap().decode(data, DecoderTrap::Ignore);
    if s.is_err(){ return None }
    Some(s.unwrap())
}

pub fn vec_to_gbk(data:Vec<u8>,a:&str)->Option<Vec<u8>>{
    to_gbk(&String::from_utf8(data).unwrap(), a)
}

pub fn to_gbk(data:&str,a:&str)->Option<Vec<u8>>{
    let t = encoding_from_whatwg_label(a);
    if t.is_none() {
        return None
    }
    let s = t.unwrap().encode(data, EncoderTrap::Strict);
    if s.is_err(){ return None }
    Some(s.unwrap())
}



///释放字符串的内存
/// 
/// pb中从返回的字符串会拷贝一份数据，而不是引用
/// Pb中引用例子如下
/// subroutine free_cstring(ulong s) LIBRARY "pjoo.dll"
pub fn free_cstring(s:PbRData){
    unsafe {
        //msg(s);
        if s.is_null() { return }
        CString::from_raw(s as * mut c_char)
    };
}

/// 返回指针
/// Pb中引用例子如下
/// FUNCTION ulong str_ptr(String a) LIBRARY "pjoo.dll"
pub fn str_ptr(s:* const c_char)->* const c_char{s}

#[cfg(test)]
mod tests {
    use super::*;
    use json::*;
    #[test]
    fn testPtr(){
        let mut s = String::from("1314");
        let a = PbValue::from_string(s);
        let b = a;
        println!("{:?}",b);
        let bb = b.into_raw();
        println!("{:?}",bb);
        unsafe {
            let a =std::ptr::read_volatile(&mut *(bb as *mut Box<Any>));
            println!("{:?}",a);
            let a1 = String::from("aaaa");
            let b1 = a1.as_ptr();
            //let c = *(b1 as *mut Box<Any>);
            //let a =std::ptr::read_volatile(&mut );
        }
    }
    #[test]
    fn testPbValue(){
        let mut s = String::from("1314");
        let a = PbValue::from_string(s);
        let b = a;
        println!("{:?}",b);
        let mut bb = b.into_raw();
        println!("bb:{:?}",bb);
        unsafe{
            let a1 = & mut bb;
            let b1 = & mut bb;
            println!("{:?}",b1);
            let a = PbValue::from_rdata_raw(& mut bb);
            println!("bb:{:?}",bb);
            println!("PbValue:{:?}",a);
            PbValue::from_rdata_raw(& mut bb);
        }

        // let a1 = String::from("aaaa");
        // let b1 = a1.as_ptr();
        // println!("{:?}",b1);
        // unsafe{
        //     let _ = PbValue::from_raw(b1 as PbRData);
        // }

    }
}
// fn msg(m:*const c_char){
//     use std::ptr::null_mut;
//     use winapi::um::winuser::{MB_OK, MessageBoxA};
//     let ret = unsafe {
//         MessageBoxA(null_mut(),m,m, MB_OK)
//     };
// }


// #[export_name="SayHello"]
// pub extern "stdcall" fn say_hello()->* const c_char{
//     "hello".as_ptr() as * const c_char
// }

// #[export_name="SayHello2"]
// pub extern "stdcall" fn say_hello2()->* const c_char{
//     let c = String::from("hello2");
//     c.as_ptr() as * const c_char
// }
// #[export_name="SayHello3"]
// pub extern "stdcall" fn say_hello3()->* const c_char{
//     let c = CStr::from_bytes_with_nul(b"hello3");
//     if c.is_err() {
//         return "\0".as_ptr() as * const c_char
//     }
//     c.unwrap().as_ptr() as * const c_char
// }





// fn into_raw(t: Vec<u8>)->* mut c_char{
//     let aa={
//         let a = Box::new(t) as Box<Any>;
//         Box::into_raw(Box::new(a)) as * mut c_char
//     };
//     aa
// } 

// fn from_raw<'a>(p:*const c_char)->Option<&'a mut Vec<u8>>{
//     let foo2 = unsafe {
//         &mut *(p as *mut Box<Any>)
//     };
//     let foo3: Option<&mut Vec<u8>> = foo2.downcast_mut(); 
//     foo3
// }

// fn str_to_pcstr(a:&str)->* const c_char{
//     let c = CStr::from_bytes_with_nul(a.as_bytes()) ;
//     if c.is_err(){
//         return "\0".as_ptr() as * const c_char
//     }
//     c.unwrap().as_ptr() as * const c_char
// }
// #[test]
// fn testcode(){
    
//     let ps = {
        
//         let s:Vec<u8> = vec![97,98,99,0];
//         Box::into_raw(Box::new(Box::new(s ) as Box<Any>)) as *const c_char
//     };
//      {
//         let foo2 = unsafe {
//             &mut *(ps as *mut Box<Any>)
//         };
//         let foo3: Option<&mut Vec<u8>> = foo2.downcast_mut(); // 如果foo2不是*const Box<Foo<Vec<i32>>>, 则foo3将会是None
//         if let Some(value) = foo3 {
//             println!("{:?}",value);
//             println!("{:?}",String::from_utf8(value.to_owned()).unwrap());
//         }
//     }

// }
// #[test]
// fn teststr_to_pcstr(){
//     use std::any::Any;
//     let s = {Box::into_raw(Box::new(Box::new(vec![1,2,3]) as Box<Any>)) as *const c_char};
//     {
//         let foo2 = unsafe {
//             &mut *(s as *mut Box<Any>)
//         };
//         let foo3: Option<&mut Vec<i32>> = foo2.downcast_mut(); // 如果foo2不是*const Box<Foo<Vec<i32>>>, 则foo3将会是None
//         if let Some(value) = foo3 {
//             println!("{:?}",value);
//             value.push(5);
//         }
//     }
//     {
//         let foo2 = unsafe {
//             &mut *(s as *mut Box<Any>)
//         };
//         let foo3: Option<&mut Vec<i32>> = foo2.downcast_mut(); // 如果foo2不是*const Box<Foo<Vec<i32>>>, 则foo3将会是None
//         if let Some(value) = foo3 {
//         println!("{:?}",value);
//         }

//     }

// }
// /// C指针与CStr 相互转换  
// /// C指针与CString 相互转换
// pub trait FromC {
    
// }