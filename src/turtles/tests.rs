#[cfg(test)]
mod tests {
    use crate::turtles::init_routes;
    use actix_web::{
        http::header::ContentType,
        test::{self, TestRequest},
        App,
    };
    use serde_json::json;
    use std::env;

    fn init() {
        env::set_var(
            "DATABASE_URL",
            format!("postgresql://splinter:cheese@localhost:5432/turtles"),
        );
        crate::test::init();
    }

    #[actix_web::test]
    async fn test_get_all_turtles() {
        init();

        let app = test::init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::get()
            .insert_header(ContentType::json())
            .uri("/turtles")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_turtle() {
        init();

        let app = test::init_service(App::new().configure(init_routes)).await;
        let req = TestRequest::get()
            .insert_header(ContentType::json())
            .uri("/turtles")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_create_turtle() {
        init();

        let app = test::init_service(App::new().configure(init_routes)).await;

        let request_body = json!({
             "name": "asd",
             "weapon": "asd",
             "email": "asd",
        });

        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .uri("/turtles")
            .set_json(request_body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_update_turtle() {
        init();

        let app = test::init_service(App::new().configure(init_routes)).await;

        let put_request_body = json!({
             "name": "UPDATE",
             "weapon": "UPDATE",
             "email": "UPDATE",
        });

        let req = test::TestRequest::put()
            .insert_header(ContentType::json())
            .uri("/turtles/1")
            .set_json(put_request_body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_delete_turtle() {
        init();

        let app = test::init_service(App::new().configure(init_routes)).await;
        let req = test::TestRequest::delete()
            .insert_header(ContentType::json())
            .uri("/turtles/2")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
