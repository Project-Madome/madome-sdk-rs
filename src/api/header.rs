/*
    REQUEST
*/
pub const MADOME_E2E_TEST: &str = "x-madome-e2e-test";
/// 사용자의 요청과 서버 상호간의 요청을 구분함
///
/// 게이트웨이를 거치면 해당 헤더가 요청에 있음
pub const MADOME_PUBLIC_ACCESS_HEADER: &str = "x-madome-public-access";
/// 응답으로 변환하지 않은 값을 받음
pub const MADOME_TAKE_ORIGIN_RESPONSE: &str = "x-madome-take-origin-response";
