use cfg_if::cfg_if;
use leptos::*;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum::{extract::Extension, routing::post, Router};
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_start::file::file_handler;
    use std::sync::Arc;
    use leptos_start::app::*;

    #[tokio::main]
    async fn main()  {
        simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");
        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let addr = leptos_options.site_address.clone();
        let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

        let app = Router::new()

        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> } )
        .fallback(file_handler)
        .layer(Extension(Arc::new(leptos_options)));

                    log!("listening on {}", &addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();



    // use actix_files::Files;
    // use actix_web::*;
    // use leptos::*;
    // use leptos_actix::{generate_route_list, LeptosRoutes};
    // use leptos_start::app::*;
    //
    // let conf = get_configuration(None).await.unwrap();
    // let addr = conf.leptos_options.site_address;
    // // Generate the list of routes in your Leptos App
    // let routes = generate_route_list(|cx| view! { cx, <App/> });
    //
    // HttpServer::new(move || {
    //     let leptos_options = &conf.leptos_options;
    //     let site_root = &leptos_options.site_root;
    //
    //     App::new()
    //         .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
    //         .leptos_routes(
    //             leptos_options.to_owned(),
    //             routes.to_owned(),
    //             |cx| view! { cx, <App/> },
    //         )
    //         .service(Files::new("/", site_root))
    //     //.wrap(middleware::Compress::default())
    // })
    // .bind(&addr)?
    }
} else {
    pub fn main() {
        // no client-side main function
        // unless we want this to work with e.g., Trunk for pure client-side testing
        // see lib.rs for hydration function instead
    }
}
}
