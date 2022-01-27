use rocket_multipart_form_data::MultipartFormDataError;

#[derive(Debug)]
pub enum TransmissionError {
    RocketError(rocket::Error),
    IOError(async_std::io::Error),
    MultipartFormError(MultipartFormDataError),
    Message(String),
}

impl From<rocket::Error> for TransmissionError {
    fn from(error: rocket::Error) -> Self {
        Self::RocketError(error)
    }
}

impl From<async_std::io::Error> for TransmissionError {
    fn from(error: async_std::io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<MultipartFormDataError> for TransmissionError {
    fn from(error: MultipartFormDataError) -> Self {
        Self::MultipartFormError(error)
    }
}

impl From<String> for TransmissionError {
    fn from(e: String) -> Self {
        Self::Message(e)
    }
}
