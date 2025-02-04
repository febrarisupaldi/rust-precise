use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, Result
};
use futures::future::{ok, Ready};
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
    sync::Mutex,
    fs::OpenOptions,
    io::Write,
};
use chrono::Local;

// Middleware struct
pub struct LoggingMiddleware;

// Middleware factory
impl<S, B> Transform<S, ServiceRequest> for LoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddlewareService { service })
    }
}

// Middleware service
pub struct LoggingMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().to_string();
        let path = req.path().to_string();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let status = res.status().as_u16();
            let duration = start.elapsed();
            let now = Local::now().format("%Y-%m-%d %H:%M:%S");

            // Log to file
            let log_entry = format!(
                "{} {} {} {} {}ms\n",
                now,
                method,
                path,
                status,
                duration.as_millis()
            );

            let log_file_name = Local::now().format("%Y-%m-%d.log").to_string();

            // Use a Mutex to safely write to the file
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(format!("log/{}", log_file_name))
                .expect("Failed to open log file");

            let file = Mutex::new(file);
            let mut file = file.lock().unwrap();
            file.write_all(log_entry.as_bytes())
                .expect("Failed to write to log file");

            Ok(res)
        })
    }
}