#![allow(dead_code)]

use serde::Serialize;

#[derive(Debug, Serialize, Copy, Clone)]
pub enum ApiCode {
    Success = 0,
    InvalidParams = 1000,
    NotFound = 1001,
    OperationFailed = 1002,
    Unauthorized = 2000,
    Forbidden = 2001,
    AuthFailed = 3000,
    UserExists = 3001,
    UserNotActivated = 3002,
    InternalError = 4000,
    DependencyError = 5000,
}

impl ApiCode {
    pub fn message(&self) -> &'static str {
        match self {
            ApiCode::Success => "成功",
            ApiCode::InvalidParams => "参数错误",
            ApiCode::NotFound => "资源不存在",
            ApiCode::OperationFailed => "操作失败",
            ApiCode::Unauthorized => "未授权",
            ApiCode::Forbidden => "无权限",
            ApiCode::AuthFailed => "认证失败",
            ApiCode::UserExists => "用户已存在",
            ApiCode::UserNotActivated => "用户未激活",
            ApiCode::InternalError => "服务异常",
            ApiCode::DependencyError => "依赖服务异常",
        }
    }

    pub fn code(&self) -> i32 {
        *self as i32
    }
}
