pub(crate) struct SorterRunner {

}

impl SorterRunner {
    fn new() -> Self {
        Self {}
    }

    #[tokio::main]
    async fn serve() {
        let routes = warp::path("hi").map(|| "Hello, World!");

        warp::serve(routes)
            .run(([127, 0, 0, 1], 3030))
            .await;
    }

    fn up() {

    }

    fn down() {

    }
}

