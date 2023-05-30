use leptos::{ev::SubmitEvent, *};
use leptos_meta::*;
use leptos_router::*;

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

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let username = username_input().expect("to exist").value();
        let password = password_input().expect("to exist").value();

        leptos::log!("username: {username}");
        leptos::log!("password: {password}");
    };

    view! { cx,
        <Title text="Login - Dalang" />
        <div class="auth-card">
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
        </div>


        <A href="" class="create-account paragraph">"Create account"</A>
    }
}

#[component]
fn RegisterPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="Register - Dalang"/>
        <p>"Hello Register!"</p>
    }
}
