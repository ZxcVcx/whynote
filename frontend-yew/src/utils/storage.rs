use web_sys::window;

pub fn store_pair(key: &str, value: &str, remember: bool) {
    let window = window().expect("no global `window` exists");
    if remember {
        let local_storage = window
            .local_storage()
            .expect("no local storage")
            .expect("local storage not available");

        local_storage
            .set_item(key, value)
            .unwrap_or_else(|_| panic!("failed to set {}", key));
        // .expect(format! {"failed to set {}", key}.as_str());
    } else {
        let session_storage = window
            .session_storage()
            .expect("no session storage")
            .expect("session storage not available");

        session_storage
            .set_item(key, value)
            .unwrap_or_else(|_| panic!("failed to set {}", key));
        // .expect(format! {"failed to set {}", key}.as_str());
    }
}

// pub fn set_token(token: &str, remember: bool) {
//     let window = window().expect("no global `window` exists");

//     if remember {
//         let local_storage = window
//             .local_storage()
//             .expect("no local storage")
//             .expect("local storage not available");

//         local_storage
//             .set_item("jwt", token)
//             .expect("failed to set token");
//     } else {
//         let session_storage = window
//             .session_storage()
//             .expect("no session storage")
//             .expect("session storage not available");

//         session_storage
//             .set_item("jwt", token)
//             .expect("failed to set token");
//     }
// }

pub fn get_pair_value(key: &str) -> Option<String> {
    let window = window().expect("no global `window` exists");

    if let Ok(Some(storage)) = window.session_storage() {
        if let Ok(Some(value)) = storage.get_item(key) {
            return Some(value);
        }
    }

    if let Ok(Some(storage)) = window.local_storage() {
        if let Ok(Some(value)) = storage.get_item(key) {
            return Some(value);
        }
    }
    None
}

// pub fn get_token() -> Option<String> {
//     let window = window().expect("no global `window` exists");

//     if let Ok(Some(token)) = window.session_storage() {
//         if let Ok(Some(token)) = token.get_item("jwt") {
//             return Some(token);
//         }
//     }

//     if let Ok(Some(token)) = window.local_storage() {
//         if let Ok(Some(token)) = token.get_item("jwt") {
//             return Some(token);
//         }
//     }
//     None
// }

pub fn remove_pair(key: &str) {
    let window = window().expect("no global `window` exists");

    if let Ok(Some(local_storage)) = window.local_storage() {
        let _ = local_storage.remove_item(key);
        // .expect("failed to remove token");
    }

    if let Ok(Some(session_storage)) = window.session_storage() {
        let _ = session_storage.remove_item(key);
        // .expect("failed to remove token");
    }
}

// pub fn remove_token() {
//     let window = window().expect("no global `window` exists");

//     if let Ok(Some(local_storage)) = window.local_storage() {
//         let _ = local_storage
//             .remove_item("jwt");
//             // .expect("failed to remove token");
//     }

//     if let Ok(Some(session_storage)) = window.session_storage() {
//         let _ = session_storage
//             .remove_item("jwt");
//             // .expect("failed to remove token");
//     }
// }
