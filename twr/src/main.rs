//! Based on https://www.youtube.com/watch?v=t2QLWQh630k&ab_channel=Quickwit

use futures::future::{ready, BoxFuture, Ready};
use std::{convert::Infallible, task::Poll, time::Duration};
use tokio::time::Sleep;
use tower::{BoxError, Layer, ServiceBuilder, ServiceExt};

#[derive(Debug)]
struct Request(String);

#[derive(Debug)]
struct Response(String);

#[derive(Debug)]
struct HelloService {}

impl tower::Service<Request> for HelloService {
    type Error = Infallible;
    type Response = Response;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn call(&mut self, req: Request) -> Self::Future {
        let fut = async move { hello(req) };
        Box::pin(fut)
    }
}

#[derive(Debug)]
struct HelloReadyService {}

impl tower::Service<Request> for HelloReadyService {
    type Error = Infallible;
    type Response = Response;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
    fn call(&mut self, req: Request) -> Self::Future {
        ready(hello(req))
    }
}

fn hello(req: Request) -> Result<Response, Infallible> {
    let msg = format!("Hello {}", req.0);
    println!("hello()");
    Ok(Response(msg))
}

#[derive(Debug)]
struct LoggingService<S> {
    inner: S,
}

impl<S, R> tower::Service<R> for LoggingService<S>
where
    S: tower::Service<R> + Send,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: R) -> Self::Future {
        let inner_fut = self.inner.call(req);
        let fut = async move {
            tracing::info!("processing request");
            let response = inner_fut.await;
            tracing::info!("done");
            response
        };
        Box::pin(fut)
    }
}

#[pin_project::pin_project]
struct LoggingFuture<F> {
    #[pin]
    inner: F,
}

#[derive(Debug)]
struct LoggingFutService<S> {
    inner: S,
}

impl<F> std::future::Future for LoggingFuture<F>
where
    F: std::future::Future,
{
    type Output = F::Output;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        let polled: std::task::Poll<_> = this.inner.poll(cx);
        if polled.is_ready() {
            tracing::info!("finished processing request");
        }
        polled
    }
}

impl<S, R> tower::Service<R> for LoggingFutService<S>
where
    S: tower::Service<R>,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = LoggingFuture<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: R) -> Self::Future {
        tracing::info!("started processing request");
        LoggingFuture {
            inner: self.inner.call(request),
        }
    }
}

#[derive(Debug)]
struct TimeoutService<S> {
    inner: S,
    timeout: Duration,
}

#[pin_project::pin_project]
struct TimeoutFuture<F> {
    #[pin]
    inner: F,
    #[pin]
    sleep: Sleep,
}

impl<F, T, E> std::future::Future for TimeoutFuture<F>
where
    F: std::future::Future<Output = Result<T, E>>,
    E: Into<crate::BoxError>,
{
    type Output = Result<T, crate::BoxError>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.inner.poll(cx) {
            Poll::Ready(result) => Poll::Ready(result.map_err(Into::into)),
            Poll::Pending => match this.sleep.poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(_) => Poll::Ready(Err(BoxError::from("timeout"))),
            },
        }
    }
}

impl<S, R> tower::Service<R> for TimeoutService<S>
where
    S: tower::Service<R>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = TimeoutFuture<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        match self.inner.poll_ready(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(result) => Poll::Ready(result.map_err(Into::into)),
        }
    }

    fn call(&mut self, request: R) -> Self::Future {
        let inner = self.inner.call(request);
        let sleep = tokio::time::sleep(self.timeout);
        TimeoutFuture { inner, sleep }
    }
}

struct TimeoutLayer {
    timeout: Duration,
}

impl<S> Layer<S> for TimeoutLayer {
    type Service = TimeoutService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        TimeoutService {
            inner,
            timeout: self.timeout,
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let r = ServiceBuilder::new()
        .layer_fn(|s| LoggingFutService { inner: s })
        .layer(TimeoutLayer {
            timeout: Duration::from_secs(1),
        })
        .service(HelloService {})
        .oneshot(Request("World".to_string()))
        .await
        .unwrap()
        .0;
    println!("{r}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::{Service, ServiceExt};

    #[tokio::test]
    async fn boxed() {
        let response = HelloService {}
            .ready()
            .await
            .unwrap()
            .call(Request("World".to_string()))
            .await
            .unwrap();
        assert_eq!(response.0, "Hello World");
    }

    #[tokio::test]
    async fn ready() {
        let response = HelloReadyService {}
            .ready()
            .await
            .unwrap()
            .call(Request("World".to_string()))
            .await
            .unwrap();
        assert_eq!(response.0, "Hello World");
    }

    #[tokio::test]
    async fn logging() {
        let response = LoggingService {
            inner: HelloService {},
        }
        .ready()
        .await
        .unwrap()
        .call(Request("World".to_string()))
        .await
        .unwrap();
        assert_eq!(response.0, "Hello World");
    }

    #[tokio::test]
    async fn logging_fut() {
        let response = LoggingFutService {
            inner: HelloService {},
        }
        .ready()
        .await
        .unwrap()
        .call(Request("World".to_string()))
        .await
        .unwrap();
        assert_eq!(response.0, "Hello World");
    }

    #[tokio::test]
    async fn timeout() {
        let response = TimeoutService {
            inner: HelloService {},
            timeout: Duration::from_secs(1),
        }
        .ready()
        .await
        .unwrap()
        .call(Request("World".to_string()))
        .await
        .unwrap();
        assert_eq!(response.0, "Hello World");
    }
}
