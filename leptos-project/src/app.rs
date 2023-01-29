use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        // use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
        use prisma_client_rust::NewClientError;
        use crate::prisma::user::Data as UserData;
        use crate::prisma::PrismaClient;

        pub fn register_server_functions() {
            _ = GetUsers::register();
        }

    } else {

    #[derive(Debug, Clone, :: serde :: Serialize, :: serde :: Deserialize)]
    pub struct UserData {
        #[serde(rename = "id")]
        pub id: String,
        #[serde(rename = "displayName")]
        pub display_name: String,
    }
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-project.css"/>
         // <Stylesheet id="leptos" href="/tailwind.css" />

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    // let add_user = create_server_multi_action::<AddUsers>(cx);

    let on_click = move |_| set_count.update(|count| *count += 1);

    // let users = create_resource(
    //     cx,
    //     move || (add_user.version().get()),
    //     move |_| get_users(cx),
    // );

    view! { cx,
        <h1 class="text-3xl">"Welcome to Leptos!"</h1>

        <button on:click=on_click>"Click Me: " {count}</button>

        // <div>
        //     { move || users.read().map(|users|  match users {
        //         Ok(users) => {
        //             users.into_iter().map(move |user| {
        //                 view! {
        //                     cx,
        //                     <div>
        //                         <p>{user.display_name}</p>
        //                     </div>
        //                 }.into_any()
        //             }).collect::<Vec<_>>()
        //         }
        //     })}
        // </div>
    }
}

#[server(GetUsers, "/api")]
pub async fn get_users() -> Result<Vec<UserData>, ServerFnError> {
    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
    let client = match client {
        Ok(client) => client,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };

    let users: Vec<UserData> = match client.user().find_many(vec![]).exec().await {
        Ok(users) => users,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };

    Ok(users)
}
//
// #[server(AddUsers, "/api")]
// pub async fn add_users(id: String) -> Result<(), ServerFnError> {
//     let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
//     let client = match client {
//         Ok(client) => client,
//         Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
//     };
//
//     let _user = match client.user().delete(user::id::equals(id)).exec().await {
//         Ok(users) => users,
//         Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
//     };
//
//     Ok(())
// }
