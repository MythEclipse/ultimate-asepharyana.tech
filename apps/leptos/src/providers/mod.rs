use leptos::*;

// --- Theme Provider ---

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        }
    }
}

#[derive(Clone)]
pub struct ThemeContext {
    pub theme: Signal<Theme>,
    pub set_theme: WriteSignal<Theme>,
}

pub fn provide_theme() {
    let (theme, set_theme) = create_signal(Theme::System);

    create_effect(move |_| {
        let theme_val = theme.get();
        if let Some(doc) = document().document_element() {
            let _ = match theme_val {
                Theme::Dark => doc.class_list().add_1("dark"),
                Theme::Light => doc.class_list().remove_1("dark"),
                Theme::System => {
                    if window()
                        .match_media("(prefers-color-scheme: dark)")
                        .ok()
                        .flatten()
                        .map(|m| m.matches())
                        .unwrap_or(false)
                    {
                         doc.class_list().add_1("dark")
                    } else {
                         doc.class_list().remove_1("dark")
                    }
                }
            };
        }
    });

    provide_context(ThemeContext { theme: theme.into(), set_theme });
}

pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect("ThemeContext not found")
}

// --- Auth Provider ---


use crate::api::types::{UserResponse, LoginRequest};
use crate::api::auth::{login as api_login, me as api_me};
use gloo_storage::{LocalStorage, Storage};

#[derive(Clone)] // Removed Copy, UserResponse is not Copy
pub struct AuthContext {
    pub user: Signal<Option<UserResponse>>,
    pub set_user: WriteSignal<Option<UserResponse>>,
    pub login: Action<LoginRequest, Result<(), String>>,
    pub logout: Action<(), ()>,
}

pub fn provide_auth() {
    let (user, set_user) = create_signal(None);

    // Load user from local storage on init
    create_effect(move |_| {
        spawn_local(async move {
            if let Ok(token) = LocalStorage::get::<String>("access_token") {
                match api_me(&token).await {
                    Ok(u) => set_user.set(Some(u)),
                    Err(_) => {
                        LocalStorage::delete("access_token");
                        set_user.set(None);
                    }
                }
            }
        });
    });

    let login = create_action(move |req: &LoginRequest| {
        let req = req.clone();
        async move {
            match api_login(req).await {
                Ok(res) => {
                    let _ = LocalStorage::set("access_token", res.access_token);
                    let _ = LocalStorage::set("refresh_token", res.refresh_token);
                    // We need to fetch the user profile or use the one from login response if available.
                    // The API login structure returns UserResponse inside LoginResponse.
                    // However, we can't easily set the signal from here due to ownership in async block.
                    // But we can return the user and let the consumer set it? 
                    // Or typically, we trigger a re-fetch or use the action value.
                    // Let's refetch me or just use the response data if we can propagate it.
                    // For simplicity, let's just trigger a re-fetch or set it if we can access the setter.
                    // We can't access set_user easily in this action closure if it's not moved properly.
                    // Actually, actions are good for this.
                    // Let's rely on the action result.
                    
                    // But wait, the context needs to update the global state.
                    // We can use create_effect on the action.value() to update the user signal.
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
    });

    let logout = create_action(move |_| async move {
        LocalStorage::delete("access_token");
        LocalStorage::delete("refresh_token");
        // We will clear the user signal via effect or manual set if we could.
    });

    // Effect to update user state on login success
    create_effect(move |_| {
        let val = login.value().get();
        if let Some(Ok(_)) = val {
             spawn_local(async move {
                if let Ok(token) = LocalStorage::get::<String>("access_token") {
                     if let Ok(u) = api_me(&token).await {
                         set_user.set(Some(u));
                     }
                }
            });
        }
    });

    // Effect to clear user on logout
    create_effect(move |_| {
        if logout.version().get() > 0 {
            set_user.set(None);
        }
    });

    provide_context(AuthContext {
        user: user.into(),
        set_user,
        login,
        logout,
    });
}


pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>().expect("AuthContext not found")
}
