use json::*;
use std::any::Any;


fn  pj_json_new ()->json::JsonValue{
    json::JsonValue::new_object()
} 

fn  pj_json_put (j:&mut json::JsonValue,key:&str,v:&str){
    j[key]=v.into();
} 

fn pj_json_free (json:json::JsonValue){
}

fn pj_json_ref<'a> (json:* mut Box<Any>)->Option<&'a mut json::JsonValue>{
    let r = ptr_ref(json);
    r.downcast_mut()
}

fn box_any(b:Box<Any>)->* mut Box<Any>{
    Box::into_raw(Box::new(b)) 
}

fn ptr_ref<'a> (json_prt:* mut Box<Any>)->&'a mut Box<Any>{
    unsafe{&mut * json_prt}
}

fn free_with_ptr (json_prt:* mut Box<Any>){
    unsafe{let _ = Box::from_raw(json_prt);}
}

#[cfg(test)]
mod tests {
    
    use super::*;
    
    fn pj_json(d:&str)->json::JsonValue{
        
        let a = parse(d);
        a.unwrap()
    }
    #[test]
    fn testJson(){
        let json = {
            let mut json = pj_json_new();
            Box::into_raw(Box::new(json))
        };
        unsafe{
            let json_ref  :&mut json::JsonValue  =&mut * json;
            let json_ref2 :&mut json::JsonValue  = &mut * json;
            pj_json_put(json_ref, "f", "100");
            println!("{:?}",json_ref);
            {
              let _j = Box::from_raw(json);
            }
            pj_json_put(json_ref2, "f2", "1002");
            println!("{}",json_ref);
        }
    }
    #[test]
    fn testBoxFree(){
        let b1 = 
        {
            let b = Box::new(Box::new(vec![1u8;2]));
            Box::into_raw(b )
        };
        unsafe {
             let foo2 =& *(b1 as *mut Box<Vec<u8>>);
             println!("foo2:{:?}",foo2);
        }
        unsafe{
           let b= Box::from_raw(b1);
           println!("{:?}",**b);
        }//free
        // unsafe{
        //    let b= Box::from_raw(b1);
        //    println!("2{:?}",**b);
        // }
    }
   
}