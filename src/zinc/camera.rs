// Documentation can be found here - https://www.foscam.es/descarga/ipcam_cgi_sdk.pdf

/// url that checks user info
pub const USER_URL: &str = "/check_user2.cgi";
/// url that allows us to control the camerq
pub const DECODER_URL: &str = "/decoder_control.cgi";
/// obtain camera param settings
pub const PARAMS_URL: &str = "/get_camera_params.cgi";

/// Holds authentication information for the FI8910W camera
pub struct Auth<'a> {
    pub username: &'a str,
    pub password: &'a str,
    /// The IP or domain name of the camera
    pub url: &'a str,
}

impl<'a> Default for Auth<'a> {
    fn default() -> Self {
        Auth {
            username: "",
            password: "",
            url: "",
        }
    }
}

pub struct Camera<'a> {
    pub auth: Auth<'a>,
}

impl<'a> Default for Camera<'a> {
    fn default() -> Self {
        Camera {
            auth: Auth::default(),
        }
    }
}
