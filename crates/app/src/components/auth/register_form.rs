use domain::user::User;
use leptos::prelude::*;
use leptos::web_sys;

/// Register form with email, password, confirm password fields, blur validation, and server error display.
/// POSTs to /api/auth/register on submit; navigates to /dashboard on success.
#[component]
pub fn RegisterForm() -> impl IntoView {
    let auth_user = use_context::<LocalResource<Option<User>>>()
        .expect("auth context required in RegisterForm");
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let confirm_password = RwSignal::new(String::new());
    let email_error: RwSignal<Option<String>> = RwSignal::new(None);
    let password_error: RwSignal<Option<String>> = RwSignal::new(None);
    let confirm_error: RwSignal<Option<String>> = RwSignal::new(None);
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

    let validate_password = move || {
        let val = password.get();
        if val.len() < 8 {
            password_error.set(Some("Password must be at least 8 characters.".to_string()));
            false
        } else {
            password_error.set(None);
            true
        }
    };

    let validate_confirm = move || {
        let pw = password.get();
        let cpw = confirm_password.get();
        if pw != cpw {
            confirm_error.set(Some("Passwords don't match. Please re-enter.".to_string()));
            false
        } else {
            confirm_error.set(None);
            true
        }
    };

    let on_email_input = move |ev: web_sys::Event| {
        email.set(event_target_value(&ev));
    };

    let on_password_input = move |ev: web_sys::Event| {
        password.set(event_target_value(&ev));
    };

    let on_confirm_input = move |ev: web_sys::Event| {
        confirm_password.set(event_target_value(&ev));
    };

    let on_submit = {
        #[cfg(target_arch = "wasm32")]
        {
            move |ev: leptos::ev::SubmitEvent| {
                ev.prevent_default();

                let email_valid = validate_email();
                let password_valid = validate_password();
                let confirm_valid = validate_confirm();
                if !email_valid || !password_valid || !confirm_valid {
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

                    let result = Request::post("/api/auth/register")
                        .header("Content-Type", "application/json")
                        .body(body.to_string())
                        .unwrap()
                        .send()
                        .await;

                    submitting.set(false);

                    match result {
                        Ok(resp) if resp.status() == 201 => {
                            auth_user.refetch();
                            let navigate = leptos_router::hooks::use_navigate();
                            navigate("/dashboard", Default::default());
                        }
                        Ok(resp) if resp.status() == 409 => {
                            error.set(Some(
                                "An account with this email already exists. Log in instead."
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
                    for="reg-email"
                    class="text-sm font-bold text-petal-white mb-1 block"
                >
                    "Email"
                </label>
                <input
                    id="reg-email"
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
                    <p id="reg-email-error" class="text-sm text-bloom-pink mt-1" role="alert">
                        {e}
                    </p>
                })}
            </div>

            // Password field
            <div class="mb-4">
                <label
                    for="reg-password"
                    class="text-sm font-bold text-petal-white mb-1 block"
                >
                    "Password"
                </label>
                <input
                    id="reg-password"
                    type="password"
                    name="password"
                    placeholder="At least 8 characters"
                    autocomplete="new-password"
                    class="bg-bark-mid border border-bark-light rounded px-4 py-3 text-petal-white placeholder-mist text-base focus:border-leaf-green focus:outline-none w-full"
                    prop:value=move || password.get()
                    on:input=on_password_input
                    on:blur=move |_: web_sys::FocusEvent| { validate_password(); }
                />
                <p class="text-sm text-mist mt-1">"At least 8 characters"</p>
                {move || password_error.get().map(|e| view! {
                    <p id="reg-password-error" class="text-sm text-bloom-pink mt-1" role="alert">
                        {e}
                    </p>
                })}
            </div>

            // Confirm password field
            <div class="mb-6">
                <label
                    for="reg-confirm-password"
                    class="text-sm font-bold text-petal-white mb-1 block"
                >
                    "Confirm password"
                </label>
                <input
                    id="reg-confirm-password"
                    type="password"
                    name="confirm_password"
                    placeholder="Repeat your password"
                    autocomplete="new-password"
                    class="bg-bark-mid border border-bark-light rounded px-4 py-3 text-petal-white placeholder-mist text-base focus:border-leaf-green focus:outline-none w-full"
                    prop:value=move || confirm_password.get()
                    on:input=on_confirm_input
                    on:blur=move |_: web_sys::FocusEvent| { validate_confirm(); }
                />
                {move || confirm_error.get().map(|e| view! {
                    <p id="reg-confirm-error" class="text-sm text-bloom-pink mt-1" role="alert">
                        {e}
                    </p>
                })}
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
                {move || if submitting.get() { "Creating account..." } else { "Create Account" }}
            </button>
        </form>
    }
}
