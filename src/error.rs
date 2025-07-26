use derive_more::{Display, Error};



#[derive(Debug, Display, Error)]
pub enum ServiceError {
    #[display(fmt = "{error_message}")]
    DBConnectionError { error_message: String },

    #[display(fmt = "{error_message}")]
    BindAddressError { error_message: String },

    #[display(fmt = "{error_message}")]
    RunServerError { error_message: String },

    #[display(fmt = "{error_message}")]
    Unauthorized { error_message: String },

    #[display(fmt = "{error_message}")]
    InternalServerError { error_message: String },

    #[display(fmt = "{error_message}")]
    BadRequest { error_message: String },

    #[display(fmt = "{error_message}")]
    NotFound { error_message: String },
}

