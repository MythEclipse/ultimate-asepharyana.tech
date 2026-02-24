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

// --- WebSocket Provider ---

use web_sys::{WebSocket, MessageEvent, ErrorEvent, CloseEvent};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::api::types::WsMessage;
use crate::api::WS_BASE_URL;

#[derive(Clone)]
pub struct WsContext {
    pub last_message: ReadSignal<Option<WsMessage>>,
    pub send_message: Callback<WsMessage>,
}

pub fn provide_ws() {
    let (last_message, set_last_message) = create_signal(None::<WsMessage>);

    // We use a stored reference to the websocket to keep it alive
    let ws_ref: StoredValue<Option<WebSocket>> = store_value(None);

    let connect = move || {
        let url = format!("{}/ws/chat", WS_BASE_URL);
        log::info!("Connecting to WebSocket: {}", url);
        
        match WebSocket::new(&url) {
            Ok(ws) => {
                ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
                
                // On message
                let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Some(text) = e.data().as_string() {
                        if let Ok(msg) = serde_json::from_str::<WsMessage>(&text) {
                            set_last_message.set(Some(msg));
                        }
                    }
                }) as Box<dyn FnMut(MessageEvent)>);
                ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget();

                // On error
                let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
                    log::error!("WebSocket error: {:?}", e);
                }) as Box<dyn FnMut(ErrorEvent)>);
                ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
                onerror_callback.forget();

                // On close
                let onclose_callback = Closure::wrap(Box::new(move |e: CloseEvent| {
                    log::warn!("WebSocket closed: {:?}", e);
                    // Reconnect logic could be added here with a delay
                }) as Box<dyn FnMut(CloseEvent)>);
                ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
                onclose_callback.forget();

                ws_ref.set_value(Some(ws));
            }
            Err(e) => log::error!("Failed to create WebSocket: {:?}", e),
        }
    };

    // Effect to connect in the browser
    create_effect(move |_| {
        connect();
    });

    let ws_ref_for_send = ws_ref; 
    let send_message = Callback::new(move |msg: WsMessage| {
        if let Some(ws) = ws_ref_for_send.get_value() {
            if ws.ready_state() == WebSocket::OPEN {
                if let Ok(text) = serde_json::to_string(&msg) {
                    let _ = ws.send_with_str(&text);
                }
            }
        }
    });

    provide_context(WsContext {
        last_message: last_message.into(),
        send_message,
    });
}

pub fn use_ws() -> WsContext {
    use_context::<WsContext>().expect("WsContext not found")
}
