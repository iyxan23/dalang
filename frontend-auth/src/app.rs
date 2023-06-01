use cfg_if::cfg_if;
use leptos::{ev::SubmitEvent, *};
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub fn register_server_functions() {
            _ = Login::register();
            _ = Register::register();
        }
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/frontend-auth.css"/>
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=|cx| view!{ cx, <AuthenticationPage/> } >
                        <Route path="login" view=|cx| view! { cx, <LoginPage/> }/>
                        <Route path="register" view=|cx| view! { cx, <RegisterPage/> }/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthResult {
    Success,
    Failed(String),
}

#[server(Login, "/api/auth", "Cbor")]
pub async fn login(
    cx: Scope,
    username: String,
    password: String,
) -> Result<AuthResult, ServerFnError> {
    let Some(res_opt) = use_context::<leptos_actix::ResponseOptions>(cx) else { panic!(); };

    // send gRPC login message to auth service
    let cookie = cookie::Cookie::build("token", format!("{username}{password}"))
        .secure(true)
        .http_only(true)
        .same_site(cookie::SameSite::Strict);

    // retrieve the host
    let cookie = if let Ok(domain) = std::env::var("HOST") {
        cookie.domain(domain)
    } else {
        // implicitly ignoring non-unicode host values, who would even put non-unicode in their
        // host anyway
        cookie
    }
    .finish();

    std::thread::sleep(std::time::Duration::from_millis(1000));

    res_opt.append_header(
        actix_web::http::header::SET_COOKIE,
        actix_web::http::header::HeaderValue::from_str(cookie.to_string().as_str()).unwrap(),
    );

    return Ok(AuthResult::Failed(String::from(
        "Invalid username or password",
    )));
}

#[server(Register, "/api/auth", "Cbor")]
pub async fn register(
    cx: Scope,
    username: String,
    password: String,
) -> Result<AuthResult, ServerFnError> {
    let Some(res_opt) = use_context::<leptos_actix::ResponseOptions>(cx) else { panic!(); };

    // send gRPC register message to auth service
    let cookie = cookie::Cookie::build("token", format!("{username}{password}"))
        .secure(true)
        .http_only(true)
        .same_site(cookie::SameSite::Strict);

    // retrieve the host
    let cookie = if let Ok(domain) = std::env::var("HOST") {
        cookie.domain(domain)
    } else {
        // implicitly ignoring non-unicode host values, who would even put non-unicode in their
        // host anyway
        cookie
    }
    .finish();

    std::thread::sleep(std::time::Duration::from_millis(1000));

    res_opt.append_header(
        actix_web::http::header::SET_COOKIE,
        actix_web::http::header::HeaderValue::from_str(cookie.to_string().as_str()).unwrap(),
    );

    return Ok(AuthResult::Success);
}

#[component]
fn AuthenticationPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <img class="dalang-logo" src="/dalang-logo.svg"/>

        <Outlet/>
    }
}

#[component]
fn LoginPage(cx: Scope) -> impl IntoView {
    let username_input: NodeRef<html::Input> = create_node_ref(cx);
    let password_input: NodeRef<html::Input> = create_node_ref(cx);

    let (error_msg_read, error_msg_write) = create_signal(cx, None);

    let login_action = create_server_action::<Login>(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        error_msg_write.set(None);

        let username = username_input().expect("to exist").value();
        let password = password_input().expect("to exist").value();

        login_action.dispatch(Login { username, password });
    };

    create_effect(cx, move |_| {
        if let Some(result) = login_action.value().get() {
            match result {
                Err(server_err) => {
                    error_msg_write.set(Some(format!("server error: {}", server_err.to_string())));
                }
                Ok(AuthResult::Failed(err)) => {
                    error_msg_write.set(Some(err));
                }

                Ok(AuthResult::Success) => {}
            }
        }
    });

    view! { cx,
        <Title text="Login - Dalang" />
        <div
            class="auth-card"
            class:pending=login_action.pending()
            class:error=move || error_msg_read().is_some()>

            <h3 class="medium">"Login"</h3>
            <form on:submit=on_submit>
                <input type="text"
                    // value=name
                    node_ref=username_input
                    placeholder="Username"
                />
                <input type="password"
                    // value=name
                    node_ref=password_input
                    placeholder="Password"
                />

                <input class="btn paragraph" style="margin-top: 1rem;" type="submit" value="Login"/>
            </form>

            <p id="error-msg">{error_msg_read}</p>
        </div>

        <A href="/register" class="create-account paragraph">"Create account"</A>
    }
}

#[component]
fn RegisterPage(cx: Scope) -> impl IntoView {
    let username_input: NodeRef<html::Input> = create_node_ref(cx);
    let password_input: NodeRef<html::Input> = create_node_ref(cx);
    let password_confirmation_input: NodeRef<html::Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let username = username_input().expect("to exist").value();
        let password = password_input().expect("to exist").value();
        let password_confirmation = password_confirmation_input().expect("to exist").value();

        leptos::log!("username: {username}");
        leptos::log!("password: {password}");
        leptos::log!("password confirm: {password_confirmation}");
    };

    view! { cx,
        <Title text="Register - Dalang" />
        <div class="auth-card">
            <h3 class="medium">"Register"</h3>
            <form on:submit=on_submit>
                <input type="text"
                    // value=name
                    node_ref=username_input
                    placeholder="Username"
                />
                <input type="password"
                    // value=name
                    node_ref=password_input
                    placeholder="Password"
                />
                <input type="password"
                    // value=name
                    node_ref=password_confirmation_input
                    placeholder="Password Confirmation"
                />

                <input class="btn paragraph" style="margin-top: 1rem;" type="submit" value="Register"/>
            </form>
        </div>

        <A href="/login" class="create-account paragraph">"Already have an account?"</A>
    }
}
