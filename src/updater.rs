//! 检查更新与下载安装包：纯 WinINet 实现（零新依赖）。
//! `InternetOpen(PRECONFIG)` 自动跟随系统代理设置；查询 GitHub Releases API
//! 获取最新版本号，下载固定地址的最新安装包到临时目录。

use std::fs::File;
use std::io::Write;
use std::path::Path;

use windows_sys::core::w;
use windows_sys::Win32::Networking::WinInet::{
    HttpQueryInfoW, InternetCloseHandle, InternetOpenUrlW, InternetOpenW, InternetReadFile,
    HTTP_QUERY_CONTENT_LENGTH, HTTP_QUERY_FLAG_NUMBER, HTTP_QUERY_STATUS_CODE,
    INTERNET_FLAG_NO_CACHE_WRITE, INTERNET_FLAG_RELOAD, INTERNET_FLAG_SECURE,
    INTERNET_OPEN_TYPE_DIRECT, INTERNET_OPEN_TYPE_PRECONFIG,
};

/// Releases API：返回最新（非预发布、非草稿）版本信息。
const RELEASES_API: &str =
    "https://api.github.com/repos/zzhzhouzhou/zzh-music-player/releases/latest";
/// 最新版本安装包的固定地址（GitHub 自动 302 到最新 Release 的同名附件），
/// 因此无需解析 API 里的附件列表。
const INSTALLER_URL: &str =
    "https://github.com/zzhzhouzhou/zzh-music-player/releases/latest/download/zzhMusicPlayer_Setup.exe";
/// GitHub API 要求请求携带 User-Agent。
const USER_AGENT: &str = "User-Agent: zzhMusicPlayer\r\n";

/// 查询最新版本号（剥掉 v 前缀，如 "1.3.0"）。
pub fn check_latest() -> Result<String, String> {
    let (status, body) = http_get_with_fallback(RELEASES_API, 4 * 1024 * 1024)?;
    match status {
        200 => {}
        403 | 429 => return Err("请求太频繁，请稍后再试".into()),
        n => return Err(format!("检查失败（HTTP {n}）")),
    }
    let text = String::from_utf8_lossy(&body);
    extract_json_string(&text, "tag_name")
        .map(|t| t.trim_start_matches(['v', 'V']).to_string())
        .ok_or_else(|| "未能解析版本信息".into())
}

/// 下载最新安装包到 `dest`，`on_progress` 按已下载比例（0~1）回调；
/// total 未知时上报 0。下载字节数与 Content-Length 不符视为不完整。
pub fn download_installer(dest: &Path, on_progress: &dyn Fn(f32)) -> Result<(), String> {
    // 代理出口常为共享 IP（匿名 API 配额易耗尽、代理亦可能断开），
    // 先走系统代理设置，失败自动回退直连。
    let download = |access: u32| -> Result<(), String> {
        let (session, request, total) = open_request(INSTALLER_URL, USER_AGENT, usize::MAX, access)?;
        let result = stream_to_file(&request, total, dest, on_progress);
        unsafe {
            InternetCloseHandle(request);
            InternetCloseHandle(session);
        }
        result
    };
    download(INTERNET_OPEN_TYPE_PRECONFIG).or_else(|_| download(INTERNET_OPEN_TYPE_DIRECT))
}

/// 系统代理优先，失败自动直连重试一次。
fn http_get_with_fallback(url: &str, max_body: usize) -> Result<(u32, Vec<u8>), String> {
    http_get(url, max_body, INTERNET_OPEN_TYPE_PRECONFIG)
        .or_else(|_| http_get(url, max_body, INTERNET_OPEN_TYPE_DIRECT))
}

/// 比较版本号：`latest` 严格大于 `current` 才返回 true。支持缺段（补 0）
/// 与可选的 v/V 前缀；非数字段按 0 处理。
pub fn version_newer(current: &str, latest: &str) -> bool {
    fn parts(v: &str) -> Vec<u64> {
        v.trim()
            .trim_start_matches(['v', 'V'])
            .split('.')
            .map(|p| p.trim().parse().unwrap_or(0))
            .collect()
    }
    let (a, b) = (parts(current), parts(latest));
    for i in 0..a.len().max(b.len()) {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        if y != x {
            return y > x;
        }
    }
    false
}

/// 从 GitHub API 的 JSON 响应中提取字符串字段（响应结构固定，无需 JSON 库）。
fn extract_json_string(body: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let rest = &body[body.find(&needle)? + needle.len()..];
    let rest = &rest[rest.find(':')? + 1..].trim_start();
    let rest = rest.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

/// 请求一个 URL：返回 (会话句柄, 请求句柄, Content-Length)。
/// 状态码 >= 400 在此统一映射为可读错误。
fn open_request(
    url: &str,
    headers: &str,
    max_body: usize,
    access: u32,
) -> Result<(*mut core::ffi::c_void, *mut core::ffi::c_void, Option<u64>), String> {
    unsafe {
        let session = InternetOpenW(
            w!("zzhMusicPlayer"),
            access,
            std::ptr::null(),
            std::ptr::null(),
            0,
        );
        if session.is_null() {
            return Err("无法初始化网络".into());
        }
        let url_w: Vec<u16> = url.encode_utf16().chain([0]).collect();
        let headers_w: Vec<u16> = headers.encode_utf16().chain([0]).collect();
        let request = InternetOpenUrlW(
            session,
            url_w.as_ptr(),
            headers_w.as_ptr(),
            headers_w.len() as u32 - 1,
            INTERNET_FLAG_RELOAD | INTERNET_FLAG_NO_CACHE_WRITE | INTERNET_FLAG_SECURE,
            0,
        );
        if request.is_null() {
            InternetCloseHandle(session);
            return Err("网络连接失败".into());
        }
        let mut status: u32 = 0;
        let mut len: u32 = 4;
        HttpQueryInfoW(
            request,
            HTTP_QUERY_STATUS_CODE | HTTP_QUERY_FLAG_NUMBER,
            &mut status as *mut u32 as *mut _,
            &mut len,
            std::ptr::null_mut(),
        );
        if status >= 400 {
            InternetCloseHandle(request);
            InternetCloseHandle(session);
            return Err(match status {
                403 | 429 => "请求太频繁，请稍后再试".into(),
                404 => "发布信息不存在".into(),
                n => format!("请求失败（HTTP {n}）"),
            });
        }
        let mut len32: u32 = 0;
        let mut len: u32 = 4;
        let total = if HttpQueryInfoW(
            request,
            HTTP_QUERY_CONTENT_LENGTH | HTTP_QUERY_FLAG_NUMBER,
            &mut len32 as *mut u32 as *mut _,
            &mut len,
            std::ptr::null_mut(),
        ) != 0
            && len32 > 0
        {
            Some(u64::from(len32))
        } else {
            None
        };
        let _ = max_body;
        Ok((session, request, total))
    }
}

/// 查询接口的读取（带 4MB 上限防御）。
fn http_get(
    url: &str,
    max_body: usize,
    access: u32,
) -> Result<(u32, Vec<u8>), String> {
    let (session, request, _) = open_request(url, USER_AGENT, max_body, access)?;
    let mut body = Vec::new();
    let result = unsafe {
        let mut buf = [0u8; 16 * 1024];
        let mut read = 0u32;
        while InternetReadFile(
            request,
            buf.as_mut_ptr() as *mut _,
            buf.len() as u32,
            &mut read,
        ) != 0
            && read > 0
        {
            body.extend_from_slice(&buf[..read as usize]);
            if body.len() > max_body {
                break;
            }
        }
        let status = {
            let mut status: u32 = 0;
            let mut len: u32 = 4;
            HttpQueryInfoW(
                request,
                HTTP_QUERY_STATUS_CODE | HTTP_QUERY_FLAG_NUMBER,
                &mut status as *mut u32 as *mut _,
                &mut len,
                std::ptr::null_mut(),
            );
            status
        };
        (status, body)
    };
    unsafe {
        InternetCloseHandle(request);
        InternetCloseHandle(session);
    }
    Ok(result)
}

/// 流式下载到文件并按比例回调进度。
fn stream_to_file(
    request: &*mut core::ffi::c_void,
    total: Option<u64>,
    dest: &Path,
    on_progress: &dyn Fn(f32),
) -> Result<(), String> {
    let mut file = File::create(dest).map_err(|e| format!("无法创建下载文件: {e}"))?;
    let mut done: u64 = 0;
    let mut last_reported = -1f32;
    let mut buf = [0u8; 64 * 1024];
    unsafe {
        let mut read = 0u32;
        while InternetReadFile(
            *request,
            buf.as_mut_ptr() as *mut _,
            buf.len() as u32,
            &mut read,
        ) != 0
            && read > 0
        {
            file.write_all(&buf[..read as usize])
                .map_err(|e| format!("写入下载文件失败: {e}"))?;
            done += u64::from(read);
            if let Some(total) = total {
                let frac = (done as f32 / total as f32).clamp(0.0, 1.0);
                // 进度只在跨过 1% 时回调，避免事件泵被刷爆。
                if (frac * 100.0) as i32 != last_reported as i32 {
                    last_reported = frac;
                    on_progress(frac);
                }
            }
        }
    }
    if let Some(total) = total
        && done != total
    {
        return Err("下载不完整，请重试".into());
    }
    file.flush().ok();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_compare() {
        assert!(version_newer("1.2.0", "1.2.1"));
        assert!(version_newer("1.2.0", "1.3.0"));
        assert!(version_newer("1.2.0", "2.0"));
        assert!(version_newer("1.2", "1.2.1"));
        assert!(!version_newer("1.2.0", "1.2.0"));
        assert!(!version_newer("1.3.0", "1.2.9"));
        assert!(!version_newer("2.0.0", "1.99.99"));
        // 剥 v 前缀与非数字段容错。
        assert!(version_newer("v1.2.0", "V1.2.1"));
        assert!(version_newer("1.2.0", "1.2.1-beta".split('-').next().unwrap()));
    }

    #[test]
    fn parse_tag_from_json() {
        // 模拟 releases/latest 响应里的字段形态。
        let body = r#"{"url":"https://api.github.com/x","tag_name": "v1.2.0",
            "name": "v1.2.0","draft":false,"prerelease":false,
            "assets":[{"name":"zzhMusicPlayer_Setup.exe"}]}"#;
        let tag = extract_json_string(body, "tag_name").unwrap();
        assert_eq!(tag.trim_start_matches(['v', 'V']), "1.2.0");
        assert!(version_newer("1.1.0", &tag.trim_start_matches(['v', 'V'])));
        assert!(extract_json_string(body, "missing_key").is_none());
        // 值里带转义引号不影响已匹配字段。
        let body2 = r#"{"a":"x\"y","tag_name":"v2.0.0"}"#;
        assert_eq!(extract_json_string(body2, "tag_name").unwrap(), "v2.0.0");
    }
}
