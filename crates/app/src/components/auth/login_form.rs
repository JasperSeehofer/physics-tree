use leptos::prelude::*;
use leptos::web_sys;

/// Login form with email/password fields, blur validation, loading state, and server error display.
/// POSTs to /api/auth/login on submit; navigates to /dashboard on success.
#[component]
pub fn LoginForm() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let email_error: RwSignal<Option<String>> = RwSignal::new(None);
    let _password_error: RwSignal<Option<String>> = RwSignal::new(None);
    let error: RwSignal<Option<String>> = RwSignal::new(None);
    let submitting = RwSignal::new(false);

    let validate_email = move || {
        let val = email.get();
        if val.is_empty() || !val.contains('@') {
            email_error.set(Some("Enter a valid email address.".to_string()));
            false
        } else {
            email_error.set(None);
            true
        }
    };

    let on_email_input = move |ev: web_sys::Event| {
        email.set(event_target_value(&ev));
    };

    let on_password_input = move |ev: web_sys::Event| {
        password.set(event_target_value(&ev));
    };

    let on_submit = {
        #[cfg(target_arch = "wasm32")]
        {
            move |ev: leptos::ev::SubmitEvent| {
                ev.prevent_default();

                let email_valid = validate_email();
                if !email_valid {
                    return;
                }

                let email_val = email.get();
                let password_val = password.get();

                submitting.set(true);
                error.set(None);

                leptos::task::spawn_local(async move {
                    use gloo_net::http::Request;

                    let body = serde_json::json!({
                        "email": email_val,
                        "password": password_val,
                    });

                    let result = Request::post("/api/auth/login")
                        .header("Content-Type", "application/json")
                        .body(body.to_string())
                        .unwrap()
                        .send()
                        .await;

                    submitting.set(false);

                    match result {
                        Ok(resp) if resp.status() == 200 => {
                            let navigate = leptos_router::hooks::use_navigate();
                            navigate("/dashboard", Default::default());
                        }
                        Ok(resp) if resp.status() == 401 => {
                            error.set(Some(
                                "Email or password is incorrect. Try again or reset your password."
                                    .to_string(),
                            ));
                        }
                        _ => {
                            error.set(Some(
                                "Something went wrong. Please try again.".to_string(),
                            ));
                        }
                    }
                });
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            move |ev: leptos::ev::SubmitEvent| {
                ev.prevent_default();
            }
        }
    };

    view! {
        <form on:submit=on_submit novalidate>
            // Email field
            <div class="mb-4">
                <label
                    for="login-email"
                    class="text-sm font-bold text-petal-white mb-1 block"
                >
                    "Email"
                </label>
                <input
                    id="login-email"
                    type="email"
                    name="email"
                    placeholder="you@example.com"
                    autocomplete="email"
                    class="bg-bark-mid border border-bark-light rounded px-4 py-3 text-petal-white placeholder-mist text-base focus:border-leaf-green focus:outline-none w-full"
                    prop:value=move || email.get()
                    on:input=on_email_input
                    on:blur=move |_: web_sys::FocusEvent| { validate_email(); }
                />
                {move || email_error.get().map(|e| view! {
                    <p id="login-email-error" class="text-sm text-bloom-pink mt-1" role="alert">
                        {e}
                    </p>
                })}
            </div>

            // Password field
            <div class="mb-6">
                <label
                    for="login-password"
                    class="text-sm font-bold text-petal-white mb-1 block"
                >
                    "Password"
                </label>
                <input
                    id="login-password"
                    type="password"
                    name="password"
                    placeholder="Your password"
                    autocomplete="current-password"
                    class="bg-bark-mid border border-bark-light rounded px-4 py-3 text-petal-white placeholder-mist text-base focus:border-leaf-green focus:outline-none w-full"
                    prop:value=move || password.get()
                    on:input=on_password_input
                />
            </div>

            // Server error
            {move || error.get().map(|e| view! {
                <p class="text-sm text-bloom-pink mb-4" role="alert">
                    {e}
                </p>
            })}

            // Submit button
            <button
                type="submit"
                class="w-full py-3 rounded-lg bg-leaf-green text-void font-bold text-sm hover:brightness-110 disabled:opacity-60"
                prop:disabled=move || submitting.get()
            >
                {move || if submitting.get() { "Logging in..." } else { "Log In" }}
            </button>
        </form>
    }
}
