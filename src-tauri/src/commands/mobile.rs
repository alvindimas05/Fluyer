    #[cfg(mobile)]
    #[tauri::command]
    pub fn request_read_audio_permission() -> bool {
        use crate::GLOBAL_APP_HANDLE;
        use tauri::plugin::PermissionState;
        use tauri_plugin_fluyer::models::PermissionType;
        use tauri_plugin_fluyer::FluyerExt;

        let app = GLOBAL_APP_HANDLE
            .get()
            .expect("Failed to get GLOBAL_APP_HANDLE");
        let res = app
            .fluyer()
            .check_permissions()
            .expect("Failed to request read audio permission");
        if res.audio != PermissionState::Granted {
            return app
                .fluyer()
                .request_permissions(Some(vec![PermissionType::Audio]))
                .unwrap()
                .audio
                == PermissionState::Granted;
        }
        true
    }
