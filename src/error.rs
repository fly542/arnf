/// 统一错误处理, 负责转换各种系统错误信息
///
///
///

#[derive(Debug)]
pub enum AError {
    IoError(std::io::Error),
    AddrParseError(std::net::AddrParseError),
}

impl std::error::Error for AError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            AError::IoError(ref e) => Some(e),
            AError::AddrParseError(ref e) => Some(e),
        }
    }
}

///实现Display的trait，并实现fmt方法
impl std::fmt::Display for AError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AError::IoError(ref e) => e.fmt(f),
            AError::AddrParseError(ref e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for AError {
    fn from(s: std::io::Error) -> Self {
        AError::IoError(s)
    }
}

/// 转换AddrParseError 错误到AError
impl From<std::net::AddrParseError> for AError {
    fn from(s: std::net::AddrParseError) -> Self {
        AError::AddrParseError(s)
    }
}
