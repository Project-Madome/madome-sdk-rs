pub struct MadomeBaseUrl {
    pub(crate) auth: String,
    pub(crate) user: String,
}

impl MadomeBaseUrl {
    pub fn stable() -> Self {
        let base_url = "https://api.madome.app".to_string();

        Self {
            auth: base_url.clone(),
            user: base_url,
        }
    }

    pub fn beta() -> Self {
        let base_url = "https://beta.api.madome.app".to_string();

        Self {
            auth: base_url.clone(),
            user: base_url,
        }
    }

    pub fn nightly() -> Self {
        let base_url = "https://test.api.madome.app".to_string();

        Self {
            auth: base_url.clone(),
            user: base_url,
        }
    }

    #[allow(dead_code)]
    pub fn internal() -> Self {
        Self {
            auth: "http://madome-auth:3112".to_string(),
            user: "http://madome-user:3112".to_string(),
        }
    }
}
