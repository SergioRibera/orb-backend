mod permission;
mod role;
mod user;

pub use permission::PermissionRepository;
pub use role::RoleRepository;
pub use user::UserRepository;

pub use crate::shared::repository::PgRepository;
