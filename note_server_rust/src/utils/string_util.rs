pub struct StringUtils {}

impl StringUtils {
    /// 将电子邮件地址转换为有效的表名
    ///
    /// @param email: 电子邮件地址
    ///
    /// @return: 有效的表名
    pub fn get_valid_table_name(email: &str) -> String {
        let mut valid_table_name = email.trim().to_string();
        valid_table_name = valid_table_name.replace("@", "_").to_string();
        valid_table_name = valid_table_name.replace(".", "_").to_string();
        valid_table_name = valid_table_name.replace(|c: char| !c.is_ascii_alphanumeric() && c != '_', "_");
        valid_table_name
    }
}
