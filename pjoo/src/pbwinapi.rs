use libc::*;
use crate::pjoo::*;
use std::ffi::CString;
use std::ptr::null_mut;
use winapi::um::processthreadsapi::{
    CreateProcessA,LPSTARTUPINFOA,LPPROCESS_INFORMATION,STARTUPINFOA,PROCESS_INFORMATION
};
use winapi::um::namedpipeapi::CreatePipe;
use winapi::um::minwinbase::{LPSECURITY_ATTRIBUTES,SECURITY_ATTRIBUTES};
use winapi::um::winnt::PHANDLE;
use winapi::um::winbase::{STARTF_USESTDHANDLES,STARTF_USESHOWWINDOW};
use winapi::um::fileapi::ReadFile;
use winapi::shared::minwindef::{LPDWORD,LPVOID,TRUE};
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::handleapi::CloseHandle;
use winapi::um::errhandlingapi::GetLastError;
///
/// 运行执行程序
/// 
/// pb定义
/// FUNCTION ulong run_cmd_hide(ref String a) LIBRARY "pjoo.dll"
/// 
pub fn  run_cmd_hide(a_s:* mut c_char)->* const c_char{
    let h_read_pipe:PHANDLE =Box::into_raw(Box::new(null_mut()));
    let h_write_pipe:PHANDLE = Box::into_raw(Box::new(null_mut()));
    let s = std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32;//return 12
    let lp_pipe_attributes:LPSECURITY_ATTRIBUTES=Box::new(SECURITY_ATTRIBUTES{
        bInheritHandle:1, lpSecurityDescriptor:null_mut(), nLength:s 
    }).as_mut();
    let a = unsafe{CreatePipe(h_read_pipe,h_write_pipe,lp_pipe_attributes,0,)};
    println!("a:{},{:?},{:?}",a,unsafe{*h_write_pipe},unsafe{*h_read_pipe});
    let cb = std::mem::size_of::<STARTUPINFOA>() as u32;//return 68
    let lp_startup_info: LPSTARTUPINFOA=Box::new(STARTUPINFOA{
        cb: cb,lpReserved: null_mut(),lpDesktop: null_mut(),lpTitle: null_mut(),
        dwX: 0,dwY: 0,dwXSize:0,dwYSize:0,dwXCountChars: 0,dwYCountChars: 0,dwFillAttribute: 0,
        dwFlags: STARTF_USESHOWWINDOW | STARTF_USESTDHANDLES,
        wShowWindow: 0,cbReserved2: 0,lpReserved2: null_mut(),hStdInput: null_mut(),
        hStdOutput: unsafe{*h_write_pipe},
        hStdError: unsafe{*h_write_pipe},
        }).as_mut();
    let lp_process_information: LPPROCESS_INFORMATION=Box::new(PROCESS_INFORMATION{
        hProcess: null_mut(),
        hThread: null_mut(),
        dwProcessId: 0,
        dwThreadId: 0,
    }).as_mut();

    let r = unsafe{
        // 创建一个进程 
        println!("创建一个进程");
        CreateProcessA(null_mut(),a_s ,null_mut(),null_mut(),
            TRUE,0,null_mut(),null_mut(),
            lp_startup_info,
            lp_process_information)
    };
    println!("r:{}",r);
    let res = match r { 
        1=> {
            //取lpProcessInformation的值
            let b =unsafe{ Box::from_raw(lp_process_information)};
            let static_ref: &'static mut PROCESS_INFORMATION = Box::leak(b);
            unsafe {
                println!("dwProcessId：{}",(*static_ref).dwProcessId);
                //等待进程退出返回
                WaitForSingleObject((*static_ref).hProcess,1000);
                
            }
            let b = unsafe{//从管道中取返回值
                let dw_read:LPDWORD = Box::new(0u32).as_mut();
                let b =CString::from_vec_unchecked(vec![0;4096]).into_raw();
                CloseHandle(*h_write_pipe);
                println!("dwProcessId2");
                let r = ReadFile(*h_read_pipe, b as LPVOID, 4096, dw_read, null_mut());
                println!("ReadFile{}",r);
                b
            };
            unsafe{
                CloseHandle((*static_ref).hProcess);
                CloseHandle((*static_ref).hThread);
            }
            println!("from_raw:{:?}",1);
            b
        },_=>{
            unsafe{
                println!("GetLastError:{}",GetLastError());
                CloseHandle(*h_write_pipe);
                CString::from_vec_unchecked(vec![0;4096]).into_raw()
            }
        }
    };
    unsafe{
        CloseHandle(*h_read_pipe);
    }
    println!("down");
    res
}

fn _run_cmd(s:& str,fgf:& str)->Vec<u8>{
    use std::process::{Command};
    let (f,last) = match s.find(fgf){
        Some(u)=>{
             s.split_at(u)
        },
        None=>(s,fgf),
    };
    let last_a :Vec<_> = last.split(fgf).collect();
    let output = Command::new(f).args(&last_a[1..])
    .output();
    match output{
        Ok(m)=>{
            println!("{}",f);
            println!("{}",last);
            match m.status.success(){
                true=>{m.stdout},
                false=>{m.stderr},
            }
        },Err(e)=>{
            use std::error::Error;
            Vec::<u8>::from(e.description())
        }
    }
    //let b = CStr::from_bytes_with_nul(&output.stdout).expect("转换成Cstr失败!");
    //let bstr =b.to_str().expect("转换成str错误!");
    //Box::new(bstr)
}
/// 导出函数
/// 
/// 执行命令行 
/// 需要调用free_cstring来释放内存
/// Pb中引用例子如下
/// FUNCTION ulong run_cmd(String a,String e) LIBRARY "pjoo.dll"
pub fn run_cmd(cmd:PbC,fgf:PbC)->* mut c_char{
    let s = cmd.to_str();
    let fgf = fgf.to_str();
    let t = _run_cmd(s,fgf);
    let cstr=unsafe{CString::from_vec_unchecked(t)};
    cstr.into_raw()
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cmd_hide(){
        let mut s = String::from("cmd /C dir\0");
        //_run_cmd_hide(s.as_mut_str()," ");
        println!("test_cmd_hide");
    }
    #[test]
    fn test_run_cmd(){
        let t = {
            let d = _run_cmd("cmd /C dir2"," ");
            println!("{:?}",d);
            let cstr=unsafe{
                CString::from_vec_unchecked(d)
            };
            //let d = d.as_slice();
            //let cstr=unsafe{
            //    CStr::from_bytes_with_nul_unchecked(d)
            //};
            //Box::into_raw(Box::new(cstr))
            cstr.into_raw()
        };
        unsafe{
            let tt = Box::from_raw(t);
            //let t = to_utf8(&d,"GBK");
            println!("t:{:?}",tt);
        }

        // unsafe{
        //     let tt = Box::from_raw(t);
        //     //let t = to_utf8(&d,"GBK");
        //     println!("t2:{:?}",tt);
        // }

    }
}

