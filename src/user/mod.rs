pub struct User<'a> {
    base_url: &'a str,
}

/// # User API
impl<'a> User<'a> {
    pub fn new(base_url: &'a str) -> Self {
        Self { base_url }
    }

    /// # Get User
    ///
    /// 주어진 `user_id`에 해당하는 `User`의 정보를 가져옵니다.
    ///
    /// * Url
    ///
    ///     * `/users/@me`
    ///
    /// * Method
    ///
    ///     `GET`
    ///
    /// * Success Response
    ///
    ///     `user`를 리턴함
    ///     * StatusCode: `200`
    ///     * Content:
    ///         ```json
    ///         { "id": "1e441e4d-f065-4f30-8c59-7e725f18ecf0"
    ///         , "name": "madome"
    ///         , "email": "user@madome.app"
    ///         , "role": 0
    ///         , "created_at": "2022-01-24T08:06:25.673860Z"
    ///         , "updated_at": "2022-01-24T08:06:25.673860Z" }
    ///         ```
    ///
    /// * Error Response
    ///
    ///     * StatusCode: `404`
    ///         * Reason: 해당 유저가 없습니다.
    ///
    /// * Example
    ///
    ///     ```bash
    ///     curl \
    ///     -X GET \
    ///     /users/@me
    ///     ```
    pub async fn get_user(&self, user_id: &str) {}

    /// # Create User
    ///
    /// 새로운 유저를 생성합니다.
    ///
    /// * Url
    ///
    ///     * `/users`
    ///
    /// * Method
    ///
    ///     `POST`
    ///
    /// * Requires Role
    ///
    ///     `Developer`
    ///
    /// * Body Parameters
    ///
    ///     * Content-Type: `application/json`
    ///     * Content:
    ///         ```json
    ///         { "name": "madome",
    ///         , "email": "user@madome.app" }
    ///         ```
    ///
    /// * Success Response
    ///
    ///     * StatusCode: `201`
    ///
    /// * Error Response
    ///
    ///     * StatusCode: `409`
    ///         * Reason: 이미 존재하는 유저입니다.
    ///
    /// * Example
    ///
    ///     ```bash
    ///     curl \
    ///     -X POST \
    ///     -H "Content-Type: application/json"
    ///     -d '{ "name": "madome", "email": "user@madome.app" }'
    ///     /users
    ///     ```
    pub async fn create_user(&self, user_name: &str, user_email: &str) {}
}
