#![deny(clippy::all)]

use napi::{Error, Result, Status};

use oogway::Oogway as _Oogway;

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct Oogway {
    inner: _Oogway
}

#[napi]
impl Oogway {
    #[napi(constructor)]
    pub fn new() -> Result<Self> {
      let inner = _Oogway::new().map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
      Ok(Self{ inner })
    }

    #[napi(setter)]
    pub fn model_name(&mut self, model_name: String){
        self.inner.model(model_name);
    }

    #[napi]
    pub async fn ask(&self, question: String) -> Result<String> {
      let response = self.inner.ask_and_wait(question).await.map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
      Ok(response.choices.iter().map(|c| c.message.content.clone().unwrap_or_default()).collect::<String>())
    }
}
