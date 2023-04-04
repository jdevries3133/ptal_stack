use leptos::*;

use common::models::LoginPayload;

use super::auth::Auth;

const USERNAME: &str = "jack";
const PASSWORD: &str = "pass";

#[component]
fn ShowAuth(cx: Scope, auth: Option<Auth>) -> impl IntoView {
    let auth_data = format!("{:?}", auth);
    view! {
        cx,
        <p>{auth_data}</p>
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (auth_signal, _) = create_signal::<Option<Auth>>(cx, None);
    let auth = create_resource(
        cx,
        move || auth_signal.get(),
        |_| async move {
            let auth_result = Auth::from_credentials(&LoginPayload {
                identifier: USERNAME.to_string(),
                password: PASSWORD.to_string(),
            })
            .await;

            if let Ok(auth_ok) = auth_result {
                Some(auth_ok)
            } else {
                None
            }
        },
    );

    view! { cx,
        <p>"hello world!"</p>
        <Suspense
            fallback=move || view! { cx, <p>"Loading"</p> }
        >
            <h2>
                "Auth token: "
                {move || {
                    auth.read(cx).map(|a| {
                        view! { cx, <ShowAuth auth=a /> }
                    })
                }}
            </h2>
        </Suspense>
    }
}
