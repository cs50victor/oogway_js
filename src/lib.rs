#![deny(clippy::all)]

use napi::{Error, Result, Status};

use oogway::Oogway as _Oogway;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

// not really optimized , prolly a better way to do this
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

    // pub fn ask(&mut self, question: String) {
    //   self.inner.ask(question)
    // }
    // pub fn ask<'a >(&mut self, py: Python<'a>, question: String) -> PyResult<&'a PyAny> {
    //     let x = self.inner.ask(question);
    //     // x
    //     pyo3_asyncio::async_std::into_coroutine(py, async move {
    //         async_std::task::sleep(Duration::from_secs(1)).await;
    //         Ok(())
    //     })
    //     // pyo3_asyncio::tokio::future_into_py(py, async move {
    //     //     async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    //     //     Ok(Python::with_gil(|py| py.None()))
    //     // })

    //     // pyo3_asyncio::async_std::into_coroutine(py, async move {
    //     //     let x = self.inner.ask(question);
    //     //     let x = x.await;
    //     //     Ok(())
    //     // })
    // }

}
