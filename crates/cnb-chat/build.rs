//! 构建脚本：扫描 references/ 目录，生成嵌入式文档映射

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("embedded_refs.rs");
    let mut out = fs::File::create(&dest_path).unwrap();

    let refs_dir = Path::new("references");
    println!("cargo:rerun-if-changed=references");

    // 收集所有 .md 文件
    let mut entries: Vec<(String, String)> = Vec::new();
    if refs_dir.is_dir() {
        collect_md_files(refs_dir, refs_dir, &mut entries);
    }

    // 生成 phf 风格的 match 函数
    writeln!(out, "/// 根据 \"service/apiname\" 路径获取嵌入的 API 文档").unwrap();
    writeln!(out, "pub fn get_embedded_doc(key: &str) -> Option<&'static str> {{").unwrap();
    writeln!(out, "    match key {{").unwrap();

    for (key, rel_path) in &entries {
        // 使用 include_str! 嵌入文件内容
        let abs_path = refs_dir.join(rel_path);
        let abs_str = abs_path
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        writeln!(out, "        \"{key}\" => Some(include_str!(\"{abs_str}\")),").unwrap();
    }

    writeln!(out, "        _ => None,").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out, "}}").unwrap();

    // 生成列出所有服务名的函数
    let mut services: Vec<String> = entries
        .iter()
        .filter_map(|(key, _)| key.split('/').next().map(String::from))
        .collect();
    services.sort();
    services.dedup();

    writeln!(out).unwrap();
    writeln!(out, "/// 列出所有 API 服务分类").unwrap();
    writeln!(out, "pub fn list_embedded_services() -> &'static [&'static str] {{").unwrap();
    write!(out, "    &[").unwrap();
    for s in &services {
        write!(out, "\"{s}\", ").unwrap();
    }
    writeln!(out, "]").unwrap();
    writeln!(out, "}}").unwrap();

    // 生成列出所有 key 的函数
    writeln!(out).unwrap();
    writeln!(out, "/// 列出所有嵌入的文档 key").unwrap();
    writeln!(out, "pub fn list_embedded_keys() -> &'static [&'static str] {{").unwrap();
    write!(out, "    &[").unwrap();
    for (key, _) in &entries {
        write!(out, "\"{key}\", ").unwrap();
    }
    writeln!(out, "]").unwrap();
    writeln!(out, "}}").unwrap();
}

/// 递归收集 .md 文件，key 格式为 "service/apiname"（无 .md 后缀）
fn collect_md_files(base: &Path, dir: &Path, entries: &mut Vec<(String, String)>) {
    let mut items: Vec<_> = fs::read_dir(dir).unwrap().filter_map(|e| e.ok()).collect();
    items.sort_by_key(|e| e.file_name());

    for entry in items {
        let path = entry.path();
        if path.is_dir() {
            collect_md_files(base, &path, entries);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            let rel = path.strip_prefix(base).unwrap();
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            let key = rel_str.strip_suffix(".md").unwrap_or(&rel_str);
            entries.push((key.to_string(), rel_str.to_string()));
        }
    }
}
