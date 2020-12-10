use tide::Request;

pub fn handler() -> tide::Server<()> {
    let mut api = tide::new();
    api.at("/").post(|mut req: Request<()>| async move {
        let cosdata = req.body_string().await.unwrap();
        println!("{}", cosdata);
        // let msguuid = Uuid::new_v4();
        // let json = json!({
        // "msg": "Hi from rust app!"
        // });
        // let body = Body::from_json(&json)?;

        // let response = Response::builder(200)
        //     .body(body)
        //     .header("Ce-Id", msguuid.to_string())
        //     .header("Ce-specversion", "0.3")
        //     .header("Ce-Source", "knative/eventing/samples/hello-world")
        //     .header("Ce-Type", "dev.knative.samples.hifromknative")
        //     .content_type(mime::JSON)
        //     .build();
        Ok("")
    });
    api
}
