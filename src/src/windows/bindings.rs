pub use windows_sys::Win32::Foundation::MAX_PATH;
pub use windows_sys::Win32::System::Diagnostics::ToolHelp::TH32CS_SNAPPROCESS;
pub use windows_sys::Win32::System::Threading::PROCESS_QUERY_INFORMATION;
pub use windows_sys::Win32::System::Threading::PROCESS_VM_READ;
pub use windows_sys::Win32::System::ProcessStatus::LIST_MODULES_ALL;
pub use windows_sys::Win32::System::Threading::PROCESS_VM_WRITE;

pub use windows_sys::Win32::Foundation::HANDLE;
pub use windows_sys::Win32::Foundation::HMODULE;
pub use windows_sys::Win32::System::ProcessStatus::MODULEINFO;
pub use windows_sys::Win32::System::Diagnostics::ToolHelp::PROCESSENTRY32W;

pub use windows_sys::Win32::System::Threading::OpenProcess;
pub use windows_sys::Win32::System::Diagnostics::ToolHelp::CreateToolhelp32Snapshot;
pub use windows_sys::Win32::System::Diagnostics::ToolHelp::Process32FirstW;
pub use windows_sys::Win32::System::Diagnostics::ToolHelp::Process32NextW;
pub use windows_sys::Win32::System::Diagnostics::Debug::ReadProcessMemory;
pub use windows_sys::Win32::System::Diagnostics::Debug::WriteProcessMemory;
pub use windows_sys::Win32::System::ProcessStatus::GetModuleFileNameExW;
pub use windows_sys::Win32::System::ProcessStatus::EnumProcessModulesEx;
pub use windows_sys::Win32::System::ProcessStatus::GetModuleBaseNameW;
pub use windows_sys::Win32::System::ProcessStatus::GetModuleInformation;
pub use windows_sys::Win32::Foundation::CloseHandle;


pub struct HandleGuard(pub HANDLE);

impl Drop for HandleGuard {
    fn drop(&mut self) {
        if self.0 != 0 as HANDLE {
            unsafe { CloseHandle(self.0); }
        }
    }
}