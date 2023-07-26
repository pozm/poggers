use std::{ffi::c_void, rc::Rc, mem::size_of};

use windows::Win32::{System::{Diagnostics::Debug::{WriteProcessMemory, ReadProcessMemory}, Memory::{VirtualProtectEx, VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, VirtualFreeEx, MEM_RELEASE, VirtualQueryEx, MEMORY_BASIC_INFORMATION}, Threading::{OpenProcess, PROCESS_ALL_ACCESS}, ProcessStatus::GetModuleFileNameExW}, Foundation::{GetLastError, HANDLE}};

use crate::{traits::{Mem, MemError}, structures::{process::{Process, External, ProcessError, U32OrString}, protections::Protections, create_snapshot::ToolSnapshot, modules::{Module, ModuleError}}, sigscan::SigScan};

use super::utils::ProcessUtils;

impl Mem for Process<External> {
    unsafe fn alter_protection(
        &self,
        addr: usize,
        size: usize,
        prot: Protections,
    ) -> Result<Protections, MemError> {
        let mut old_protect = Default::default();
        let res = unsafe {
            VirtualProtectEx(
                self.get_handle(),
                addr as *const c_void,
                size,
                prot.native(),
                &mut old_protect,
            )
        };
        if res.as_bool() {
            Ok(old_protect.0.into())
        } else {
            // plan to match in the future, cba atm
            let e = unsafe { GetLastError() };
            {
                println!("Error: {:?}", e);
            }
            Err(MemError::ProtectFailure(addr,size,prot))
        }
    }
    unsafe fn raw_read(&self, addr: usize, data: *mut u8, size: usize) -> Result<(), MemError> {
        let res = ReadProcessMemory(
            self.get_handle(),
            addr as *const c_void,
            data as *mut _,
            size,
            Some(&mut 0),
        );

        if res.as_bool() {
            Ok(())
        } else {
            Err(MemError::ReadFailure(addr))
        }
    }
    unsafe fn raw_write(&self, addr: usize, data: *const u8, size: usize) -> Result<(), MemError> {
        let res = WriteProcessMemory(
            self.get_handle(),
            addr as *const c_void,
            data as *const _,
            size,
            Some(&mut 0),
        );
        if res.as_bool() {
            Ok(())
        } else {
            Err(MemError::WriteFailure(addr))
        }
    }
    #[must_use = "keep the virtalloc alive to keep the memory allocated"]
    unsafe fn raw_virtual_alloc(&self, addr: usize, size: usize, prot: Protections) -> Result<(),MemError> {
        let alloc_ret = VirtualAllocEx(
            self.get_handle(),
            Some(addr as *mut c_void),
            size,
            MEM_COMMIT | MEM_RESERVE,
            prot.native(),
        );
        if alloc_ret.is_null() {
            Err(MemError::AllocFailure(size, addr))
        } else {
            Ok(())
        }
    }
    unsafe fn raw_virtual_free(&self, addr:usize, size:usize) -> Result<(),MemError> {
        let is_ok = VirtualFreeEx(self.get_handle(),addr as *mut c_void, size, MEM_RELEASE);
        if is_ok.as_bool() {
            Ok(())
        }
        else {
            Err(MemError::FreeFailure(addr,size))
        }
    }
    unsafe fn raw_query(&self, addr : usize) -> MEMORY_BASIC_INFORMATION {
        let mut info =  MEMORY_BASIC_INFORMATION {
            RegionSize : 0x4096,
            ..Default::default()
        };
        VirtualQueryEx(self.get_handle(), Some(addr as *const c_void), &mut info, size_of::<MEMORY_BASIC_INFORMATION>());
        info
    }
}
impl Process<External> {
    fn open_handle(process_id: u32) -> Result<HANDLE,ProcessError> {
        let hndl = unsafe {
            OpenProcess(PROCESS_ALL_ACCESS, false, process_id).or(Err(
                ProcessError::UnableToOpenProcess(U32OrString::U32(process_id)),
            ))?
        };
        if hndl.is_invalid() {
            Err(ProcessError::UnableToOpenProcess(U32OrString::U32(process_id)))
        } else {
            Ok(hndl)
        }
    }

    fn get_name_from_mod(hndl : HANDLE) -> String {
        let mut name_buf = widestring::U16String::new();
        unsafe { GetModuleFileNameExW(hndl,None, name_buf.as_mut_slice()) } ;
        name_buf.to_string().unwrap()
    }
    /// finds the process from a pid
    pub fn find_from_pid(pid:u32) -> Result<Self,ProcessError> {
        let open_hndl = Self::open_handle(pid)?;
        let name = Self::get_name_from_mod(open_hndl);
        Ok(Self {
            handl: Some(open_hndl.0),
            pid,
            name,
            mrk: Default::default(),
        })
    }
    /// finds the process from a name
    pub fn find_by_name(name:&str) -> Result<Self,ProcessError> {
        let mut snapshot = ToolSnapshot::new_process().unwrap();
        let res = snapshot
            .find(|process| process.exe_path == name)
            .ok_or(ProcessError::UnableToFindProcess(U32OrString::String(name.to_string())))?;
        Self::find_from_pid(res.id)
    }
}

impl ProcessUtils for Process<External> {
    fn get_module(&self, name:&str) -> Result<Module<Self>,ModuleError> where Self: Sized + SigScan {
        let mut snapshot = ToolSnapshot::new_module(Some(self.pid)).unwrap();
        let res = snapshot
            .find(|module| module.name == name)
            .ok_or(ModuleError::NoModuleFound(name.to_string()))?;
        let owner = Rc::new(self.clone());
        Ok(Module{
            base_address: res.base_address,
            size: res.size,
            name: res.name,
            handle: Some(res.handle.0),
            owner
        })

    }
}
impl Clone for Process<External> {
    fn clone(&self) -> Self {
        Self {
            handl: Some(self.get_handle().0),
            pid: self.pid,
            name: self.name.clone(),
            mrk: Default::default(),
        }
    }
}
impl TryFrom<u32> for Process<External> {
    type Error = ProcessError;
    fn try_from(pid: u32) -> Result<Self,Self::Error> {
        Self::find_from_pid(pid)
    }
}
impl TryFrom<&str> for Process<External> {
    type Error = ProcessError;
    fn try_from(name: &str) -> Result<Self,Self::Error> {
        Self::find_by_name(name)
    }
}
impl SigScan for Process<External> {}