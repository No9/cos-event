use tide::Request;

pub fn handler() -> tide::Server<()> {
    let mut api = tide::new();
    api.at("/").post(|mut req: Request<()>| async move {
        let cosdata = req.body_string().await.unwrap();
        println!("{}", cosdata);
        Ok("")
    });
    api
}
