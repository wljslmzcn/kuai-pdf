use std::path::Path;
use lopdf::{Object, Dictionary};
use crate::pdf_utils::PdfUtils;

pub struct Security;

impl Security {
    pub fn encrypt_pdf(
        input_path: &Path,
        output_path: &Path,
        user_password: &str,
        owner_password: &str,
        permissions: &[&str],
    ) -> Result<(), String> {
        let mut doc = PdfUtils::open_pdf(input_path)?;

        // 权限掩码
        let mut perm: i64 = -3904;
        for p in permissions {
            match *p {
                "print" => perm |= 4,
                "copy" => perm |= 16,
                "modify" => perm |= 8,
                _ => {}
            }
        }
        let perm_u32 = perm as u32;

        let file_id = get_file_id(&doc, input_path);

        // 1. 算O值
        let o_value = compute_o_value(owner_password.as_bytes(), user_password.as_bytes());

        // 2. 算加密密钥（完全按照lopdf解密代码的get_encryption_key）
        let encryption_key = compute_encryption_key(user_password.as_bytes(), &o_value, perm_u32, &file_id, 3);

        // 3. 算U值（完全按照lopdf的compute_user_password）
        let u_value = compute_user_password(&encryption_key, 3, &file_id);

        // 4. 创建加密字典
        let mut encrypt_dict = Dictionary::new();
        let _ = encrypt_dict.set("Filter", Object::Name(b"Standard".to_vec()));
        let _ = encrypt_dict.set("V", Object::Integer(2));
        let _ = encrypt_dict.set("R", Object::Integer(3));
        let _ = encrypt_dict.set("Length", Object::Integer(128));
        let _ = encrypt_dict.set("P", Object::Integer(perm as i64));
        let _ = encrypt_dict.set("O", Object::String(o_value, lopdf::StringFormat::Literal));
        let _ = encrypt_dict.set("U", Object::String(u_value.clone(), lopdf::StringFormat::Literal));

        let encrypt_ref = doc.add_object(Object::Dictionary(encrypt_dict));
        let _ = doc.trailer.set("Encrypt", Object::Reference(encrypt_ref));

        let id_array = vec![
            Object::String(file_id.clone(), lopdf::StringFormat::Literal),
            Object::String(file_id.clone(), lopdf::StringFormat::Literal),
        ];
        let _ = doc.trailer.set("ID", Object::Array(id_array));

        // 5. 加密所有字符串和流（完全按照lopdf的decrypt_object的逆操作）
        let object_ids: Vec<_> = doc.objects.keys().cloned().collect();
        for obj_id in &object_ids {
            if *obj_id == encrypt_ref { continue; }

            if let Some(obj) = doc.objects.get_mut(obj_id) {
                match obj {
                    Object::String(ref mut s, ref mut fmt) => {
                        let key = per_object_key(&encryption_key, obj_id.0, obj_id.1);
                        rc4_crypt(&key, s);
                        *fmt = lopdf::StringFormat::Hexadecimal;
                    }
                    Object::Stream(ref mut stream) => {
                        let key = per_object_key(&encryption_key, obj_id.0, obj_id.1);
                        rc4_crypt(&key, &mut stream.content);
                    }
                    _ => {}
                }
            }
        }

        PdfUtils::save_pdf(&mut doc, output_path)
    }

    pub fn decrypt_pdf(
        input_path: &Path,
        output_path: &Path,
        password: &str,
    ) -> Result<(), String> {
        let mut doc = PdfUtils::open_pdf(input_path)?;

        // 获取加密信息
        let encrypt_ref = doc.trailer.get(b"Encrypt")
            .and_then(|v| v.as_reference())
            .map_err(|_| "找不到加密字典".to_string())?;

        let encrypt_obj = doc.get_object(encrypt_ref)
            .map_err(|e| format!("读取加密字典失败: {}", e))?;

        let encrypt_dict = match encrypt_obj {
            lopdf::Object::Dictionary(d) => d,
            _ => return Err("加密字典格式错误".to_string()),
        };

        let revision = encrypt_dict.get(b"R")
            .and_then(|v| v.as_i64())
            .unwrap_or(2);
        let o_value = encrypt_dict.get(b"O")
            .and_then(|v| v.as_str())
            .map_err(|_| "读取O值失败".to_string())?;
        let p_value = encrypt_dict.get(b"P")
            .and_then(|v| v.as_i64())
            .unwrap_or(-4) as u32;

        let file_id = get_file_id(&doc, input_path);

        // 计算加密密钥
        let encryption_key = compute_encryption_key(password.as_bytes(), o_value, p_value, &file_id, revision);

        // 验证密码
        let u_stored = encrypt_dict.get(b"U")
            .and_then(|v| v.as_str())
            .map_err(|_| "读取U值失败".to_string())?;
        let u_computed = compute_user_password(&encryption_key, revision, &file_id);
        if u_stored[..16.min(u_stored.len())] != u_computed[..16.min(u_computed.len())] {
            return Err("密码错误".to_string());
        }

        // 解密所有字符串和流
        let object_ids: Vec<_> = doc.objects.keys().cloned().collect();
        for obj_id in &object_ids {
            if *obj_id == encrypt_ref { continue; }

            if let Some(obj) = doc.objects.get_mut(obj_id) {
                match obj {
                    Object::String(ref mut s, ref mut fmt) => {
                        let key = per_object_key(&encryption_key, obj_id.0, obj_id.1);
                        rc4_crypt(&key, s);
                        // 恢复为Literal格式
                        *fmt = lopdf::StringFormat::Literal;
                    }
                    Object::Stream(ref mut stream) => {
                        let key = per_object_key(&encryption_key, obj_id.0, obj_id.1);
                        rc4_crypt(&key, &mut stream.content);
                    }
                    _ => {}
                }
            }
        }

        // 移除加密字典
        let _ = doc.trailer.remove(b"Encrypt");

        PdfUtils::save_pdf(&mut doc, output_path)
    }

    pub fn get_metadata(input_path: &Path) -> Result<serde_json::Value, String> {
        let doc = PdfUtils::open_pdf(input_path)?;
        let mut title = String::new();
        let mut author = String::new();
        let mut subject = String::new();
        let mut keywords = String::new();

        if let Ok(info_val) = doc.trailer.get(b"Info") {
            if let Ok(info_ref) = info_val.as_reference() {
                if let Ok(obj) = doc.get_object(info_ref) {
                    if let lopdf::Object::Dictionary(ref dict) = obj {
                        if let Ok(v) = dict.get(b"Title") {
                            if let lopdf::Object::String(s, _) = v {
                                title = String::from_utf8_lossy(s).to_string();
                            }
                        }
                        if let Ok(v) = dict.get(b"Author") {
                            if let lopdf::Object::String(s, _) = v {
                                author = String::from_utf8_lossy(s).to_string();
                            }
                        }
                        if let Ok(v) = dict.get(b"Subject") {
                            if let lopdf::Object::String(s, _) = v {
                                subject = String::from_utf8_lossy(s).to_string();
                            }
                        }
                        if let Ok(v) = dict.get(b"Keywords") {
                            if let lopdf::Object::String(s, _) = v {
                                keywords = String::from_utf8_lossy(s).to_string();
                            }
                        }
                    }
                }
            }
        }

        Ok(serde_json::json!({
            "title": title,
            "author": author,
            "subject": subject,
            "keywords": keywords,
        }))
    }

    pub fn update_metadata(
        input_path: &Path,
        output_path: &Path,
        metadata: &serde_json::Value,
    ) -> Result<(), String> {
        let mut doc = PdfUtils::open_pdf(input_path)?;
        let mut info_dict = lopdf::Dictionary::new();

        if let Some(title) = metadata.get("title").and_then(|v| v.as_str()) {
            let _ = info_dict.set("Title", lopdf::Object::String(title.as_bytes().to_vec(), lopdf::StringFormat::Literal));
        }
        if let Some(author) = metadata.get("author").and_then(|v| v.as_str()) {
            let _ = info_dict.set("Author", lopdf::Object::String(author.as_bytes().to_vec(), lopdf::StringFormat::Literal));
        }
        if let Some(subject) = metadata.get("subject").and_then(|v| v.as_str()) {
            let _ = info_dict.set("Subject", lopdf::Object::String(subject.as_bytes().to_vec(), lopdf::StringFormat::Literal));
        }
        if let Some(keywords) = metadata.get("keywords").and_then(|v| v.as_str()) {
            let _ = info_dict.set("Keywords", lopdf::Object::String(keywords.as_bytes().to_vec(), lopdf::StringFormat::Literal));
        }

        let info_ref = doc.add_object(lopdf::Object::Dictionary(info_dict));
        let _ = doc.trailer.set("Info", lopdf::Object::Reference(info_ref));
        PdfUtils::save_pdf(&mut doc, output_path)
    }

    pub fn clear_metadata(
        input_path: &Path,
        output_path: &Path,
    ) -> Result<(), String> {
        let mut doc = PdfUtils::open_pdf(input_path)?;
        let info_dict = lopdf::Dictionary::new();
        let info_ref = doc.add_object(lopdf::Object::Dictionary(info_dict));
        let _ = doc.trailer.set("Info", lopdf::Object::Reference(info_ref));
        PdfUtils::save_pdf(&mut doc, output_path)
    }
}

// ==================== PDF 加密辅助函数 ====================
// 完全按照 lopdf encryption.rs 的对称实现

fn md5_hash(data: &[u8]) -> [u8; 16] {
    use md5::{Md5, Digest};
    let mut hasher = Md5::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut out = [0u8; 16];
    out.copy_from_slice(&result);
    out
}

fn rc4_crypt(key: &[u8], data: &mut [u8]) {
    // RC4 init
    let mut s: Vec<u8> = (0..=255).collect();
    let mut j: usize = 0;
    for i in 0..=255 {
        j = (j + s[i] as usize + key[i % key.len()] as usize) % 256;
        s.swap(i, j);
    }
    // RC4 crypt
    let mut i: usize = 0;
    j = 0;
    for byte in data.iter_mut() {
        i = (i + 1) % 256;
        j = (j + s[i] as usize) % 256;
        s.swap(i, j);
        let k = s[(s[i] as usize + s[j] as usize) % 256];
        *byte ^= k;
    }
}

const PAD_BYTES: [u8; 32] = [
    0x28, 0xBF, 0x4E, 0x5E, 0x4E, 0x75, 0x8A, 0x41,
    0x64, 0x00, 0x4E, 0x56, 0xFF, 0xFA, 0x01, 0x08,
    0x2E, 0x2E, 0x00, 0xB6, 0xD0, 0x68, 0x3E, 0x80,
    0x2F, 0x0C, 0xA9, 0xFE, 0x64, 0x53, 0x69, 0x7A,
];

/// 计算加密密钥 - 完全按照 lopdf get_encryption_key 的逻辑
/// Algorithm 3.2:
/// 1. padded_password + O + P(LE 4bytes) + file_id
/// 2. MD5 + truncate to key_len, repeat50 times (total51)
fn compute_encryption_key(password: &[u8], o_value: &[u8], permissions: u32, file_id: &[u8], revision: i64) -> Vec<u8> {
    let key_len: usize = 16; // 128-bit

    // Step 1: pad password to 32 bytes
    let mut key = Vec::with_capacity(128);
    let password_len = std::cmp::min(password.len(), 32);
    key.extend_from_slice(&password[0..password_len]);
    key.extend_from_slice(&PAD_BYTES[0..32 - password_len]);

    // Step 2: append O value
    key.extend_from_slice(o_value);

    // Step 3: append P (4 bytes LE)
    key.extend_from_slice(&permissions.to_le_bytes());

    // Step 4: append file ID
    key.extend_from_slice(file_id);

    // Step 5: if revision >= 4, append 0xFFFFFFFF
    if revision >= 4 {
        key.extend_from_slice(&[0xFF_u8, 0xFF, 0xFF, 0xFF]);
    }

    // Step 6: MD5 hash, truncate to key_len, repeat (51 times for R=3)
    let n_hashes = if revision < 3 { 1 } else { 51 };
    for _ in 0..n_hashes {
        let digest = md5_hash(&key);
        key.truncate(key_len);
        key.copy_from_slice(&digest[..key_len]);
    }

    key
}

/// 计算O值 - Algorithm 3.3
/// 完全按照 lopdf 逻辑
fn compute_o_value(owner_password: &[u8], user_password: &[u8]) -> Vec<u8> {
    // Step 1: pad owner password, MD5
    let mut padded_owner = owner_password.to_vec();
    padded_owner.resize(32, 0);
    for i in owner_password.len()..32 {
        padded_owner[i] = PAD_BYTES[i - owner_password.len()];
    }
    let md5_owner = md5_hash(&padded_owner);

    // Step 2: pad user password
    let mut padded_user = user_password.to_vec();
    padded_user.resize(32, 0);
    for i in user_password.len()..32 {
        padded_user[i] = PAD_BYTES[i - user_password.len()];
    }

    // Step 3: RC4 encrypt with md5_owner
    rc4_crypt(&md5_owner, &mut padded_user);

    // Step 4: R=3, 19 rounds
    for i in 1..=19 {
        let mut round_key = md5_owner;
        for b in round_key.iter_mut() {
            *b ^= i as u8;
        }
        rc4_crypt(&round_key, &mut padded_user);
    }

    padded_user
}

/// 计算U值 - 完全按照 lopdf compute_user_password 的逻辑
/// Algorithm 3.5 (R >= 3):
/// 1. hash = MD5(PAD_BYTES + file_id_0)
/// 2. encrypted = RC4(hash, key)
/// 3. 19 rounds of XOR + RC4
/// 4. append PAD_BYTES[0..16]
fn compute_user_password(key: &[u8], revision: i64, file_id: &[u8]) -> Vec<u8> {
    if revision == 2 {
        // Algorithm 3.4: just RC4(PAD_BYTES, key)
        let mut result = PAD_BYTES.to_vec();
        rc4_crypt(key, &mut result);
        return result;
    }

    // Algorithm 3.5 (R >= 3)
    // Step 1: MD5(PAD_BYTES + file_id_0)
    let mut hash_input = Vec::new();
    hash_input.extend_from_slice(&PAD_BYTES);
    hash_input.extend_from_slice(file_id);
    let hash = md5_hash(&hash_input);

    // Step 2: RC4 encrypt hash with key
    let mut encrypted_hash = hash.to_vec();
    rc4_crypt(key, &mut encrypted_hash);

    // Step 3: 19 rounds
    let mut temp_key = vec![0u8; key.len()];
    for i in 1..=19 {
        for (k, t) in key.iter().zip(temp_key.iter_mut()) {
            *t = k ^ (i as u8);
        }
        rc4_crypt(&temp_key, &mut encrypted_hash);
    }

    // Step 4: append PAD_BYTES[0..16]
    encrypted_hash.extend_from_slice(&PAD_BYTES[0..16]);

    encrypted_hash
}

/// 计算每个对象的加密密钥 - 完全按照 lopdf decrypt_object 的逆操作
fn per_object_key(encryption_key: &[u8], obj_num: u32, gen_num: u16) -> Vec<u8> {
    let mut builder = Vec::<u8>::with_capacity(encryption_key.len() + 5);
    builder.extend_from_slice(encryption_key);
    builder.extend_from_slice(&obj_num.to_le_bytes()[..3]);
    builder.extend_from_slice(&gen_num.to_le_bytes()[..2]);

    let key_len = std::cmp::min(builder.len(), 16);
    let digest = md5_hash(&builder);
    digest[..key_len].to_vec()
}

fn get_file_id(doc: &lopdf::Document, path: &Path) -> Vec<u8> {
    if let Ok(id_val) = doc.trailer.get(b"ID") {
        if let lopdf::Object::Array(arr) = id_val {
            if let Some(lopdf::Object::String(s, _)) = arr.first() {
                return s.clone();
            }
        }
    }
    if let Ok(data) = std::fs::read(path) {
        md5_hash(&data).to_vec()
    } else {
        md5_hash(b"kuai-pdf-default-id").to_vec()
    }
}
