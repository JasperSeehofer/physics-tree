use domain::user::User;
use leptos::prelude::*;

/// Get the display initial for an avatar (first char of display_name or email prefix).
fn get_initial(user: &User) -> String {
    if let Some(name) = &user.display_name {
        name.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default()
    } else {
        user.email
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_default()
    }
}

/// Avatar button with dropdown menu for logged-in users.
/// Shows: Dashboard link, Settings link, Log Out button.
#[component]
pub fn AvatarDropdown(user: User) -> impl IntoView {
    let open = RwSignal::new(false);
    let initial = get_initial(&user);

    let toggle = move |_: leptos::ev::MouseEvent| open.update(|v| *v = !*v);

    let logout = {
        #[cfg(target_arch = "wasm32")]
        {
            move |_: leptos::ev::MouseEvent| {
                leptos::task::spawn_local(async move {
                    use gloo_net::http::Request;
                    let _ = Request::post("/api/auth/logout").send().await;
                    // Reload page to reset auth context
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().reload();
                    }
                });
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            move |_: leptos::ev::MouseEvent| {}
        }
    };

    // Close on Escape key
    #[cfg(target_arch = "wasm32")]
    {
        let open_for_keydown = open;
        window_event_listener(leptos::ev::keydown, move |ev: web_sys::KeyboardEvent| {
            if ev.key() == "Escape" {
                open_for_keydown.set(false);
            }
        });
    }

    view! {
        <div class="relative">
            // Avatar button
            <button
                class="w-8 h-8 rounded-full bg-leaf-green/20 ring-2 ring-leaf-green flex items-center justify-center focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-leaf-green"
                aria-label="Account menu"
                on:click=toggle
            >
                <span class="text-sm font-bold text-leaf-green">
                    {initial.clone()}
                </span>
            </button>

            // Dropdown panel
            {move || open.get().then(|| view! {
                <div class="absolute right-0 mt-2 bg-bark-dark border border-bark-light rounded-card shadow-lg min-w-[160px] py-2 z-50">
                    <a
                        href="/dashboard"
                        class="px-4 py-2 text-sm text-petal-white hover:bg-bark-mid cursor-pointer block"
                        on:click=move |_: leptos::ev::MouseEvent| open.set(false)
                    >
                        "Dashboard"
                    </a>
                    <a
                        href="#"
                        class="px-4 py-2 text-sm text-petal-white hover:bg-bark-mid cursor-pointer block"
                        on:click=move |_: leptos::ev::MouseEvent| open.set(false)
                    >
                        "Settings"
                    </a>
                    <button
                        class="px-4 py-2 text-sm text-bloom-pink hover:bg-bark-mid cursor-pointer block w-full text-left"
                        on:click=logout
                    >
                        "Log Out"
                    </button>
                </div>
            })}
        </div>
    }
}
