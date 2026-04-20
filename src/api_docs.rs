use utoipa::OpenApi;

use crate::modules;

pub const SCALAR_HTML: &str = r#"<!doctype html>
<html>
  <head>
    <title>Orb API</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body>
    <script
      id="api-reference"
      type="application/json"
      data-configuration='{
        "theme": "deepSpace",
        "layout": "modern",
        "showSidebar": true,
        "defaultOpenFirstTag": true,
        "hideModels": false,
        "withDefaultFonts": true,
        "operationTitleSource": "summary",
        "hideClientButton": false,
        "isEditable": false,
      }'
    >$spec</script>
    <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
  </body>
</html>"#;

#[derive(OpenApi)]
#[openapi(
    info(title = "Orb API", version = "0.1.0"),
    paths(
        // Auth
        modules::auth::handler::login,
        modules::auth::handler::refresh,
        // IAM
        modules::iam::handler::register,
        modules::iam::handler::list_users,
        modules::iam::handler::create_role,
        modules::iam::handler::list_roles,
        modules::iam::handler::assign_role_to_user,
        modules::iam::handler::assign_permission_to_role,
        modules::iam::handler::create_permission,
        modules::iam::handler::list_permissions,
        // Stores
        modules::stores::handler::create_store,
        modules::stores::handler::list_stores,
        modules::stores::handler::create_device,
        modules::stores::handler::list_devices,
        // Catalog
        modules::catalog::handler::create_category,
        modules::catalog::handler::list_categories,
        modules::catalog::handler::create_product,
        modules::catalog::handler::list_products,
        modules::catalog::handler::assign_category,
        modules::catalog::handler::set_price,
        // Inventory
        modules::inventory::handler::create_movement,
        modules::inventory::handler::list_movements_by_store,
        modules::inventory::handler::get_stock,
        // Sales
        modules::sales::handler::create_sale,
        modules::sales::handler::list_sales_by_store,
        modules::sales::handler::get_sale,
        // Cash
        modules::cash::handler::open_session,
        modules::cash::handler::close_session,
        modules::cash::handler::list_sessions,
        modules::cash::handler::get_session,
        // Customers
        modules::customers::handler::create_customer,
        modules::customers::handler::list_customers,
        modules::customers::handler::get_customer,
    ),
    components(schemas(
        // Auth
        modules::auth::model::LoginRequest,
        modules::auth::model::LoginResponse,
        modules::auth::model::RefreshRequest,
        // IAM
        modules::iam::model::RegisterRequest,
        modules::iam::model::RegisterResponse,
        modules::iam::model::UserResponse,
        modules::iam::model::CreateRoleRequest,
        modules::iam::model::RoleResponse,
        modules::iam::model::CreatePermissionRequest,
        modules::iam::model::PermissionResponse,
        // Stores
        modules::stores::model::CreateStoreRequest,
        modules::stores::model::StoreResponse,
        modules::stores::model::CreateDeviceRequest,
        modules::stores::model::DeviceResponse,
        // Catalog
        modules::catalog::model::CreateCategoryRequest,
        modules::catalog::model::CategoryResponse,
        modules::catalog::model::CreateProductRequest,
        modules::catalog::model::ProductResponse,
        modules::catalog::model::SetProductPriceRequest,
        modules::catalog::model::ProductPriceResponse,
        // Inventory
        modules::inventory::model::CreateMovementRequest,
        modules::inventory::model::MovementResponse,
        modules::inventory::model::StockResponse,
        // Sales
        modules::sales::model::SaleItemInput,
        modules::sales::model::PaymentInput,
        modules::sales::model::CreateSaleRequest,
        modules::sales::model::SaleResponse,
        modules::sales::model::SaleItemResponse,
        modules::sales::model::PaymentResponse,
        modules::sales::model::SaleDetailResponse,
        // Cash
        modules::cash::model::OpenSessionRequest,
        modules::cash::model::CloseSessionRequest,
        modules::cash::model::SessionResponse,
        // Customers
        modules::customers::model::CreateCustomerRequest,
        modules::customers::model::CustomerResponse,
    )),
    tags(
        (name = "Auth", description = "Authentication"),
        (name = "IAM", description = "Users, roles and permissions"),
        (name = "Stores", description = "Stores and devices"),
        (name = "Catalog", description = "Products and categories"),
        (name = "Inventory", description = "Stock movements"),
        (name = "Sales", description = "Sales and payments"),
        (name = "Cash", description = "Cash sessions"),
        (name = "Customers", description = "Customer management"),
    )
)]
pub struct ApiDoc;
