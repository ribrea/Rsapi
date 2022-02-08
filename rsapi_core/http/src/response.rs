#[macro_export]
macro_rules! response_json {
    ($body:expr) => {
     Ok(Response::builder()
        .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .status(StatusCode::OK)
        .body(Body::from($body.dump()))
        .unwrap())
    };
    ($body:expr, $status_code:expr) => {
     Ok(Response::builder()
        .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .status($status_code)
        .body(Body::from($body.dump()))
        .unwrap())
    };
    ($body:expr, $status_code:expr, $headers:expr) => {
        {
        let mut m =  Response::builder()
         .status($status_code);
        for (k, v) in $headers {
            m = m.header(k, v);
        }
        let m = m.body(Body::from($body.dump()));
        let m = m.unwrap();
        let m = Ok(m);
        m
        }};
    () => {
        Ok( Response::builder()
            .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .status(StatusCode::OK)
            .body(Body::from("{}"))
            .unwrap())
    };
}

