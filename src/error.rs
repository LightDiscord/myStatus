/**
 * Author: LightDiscord <arnaud@lightdiscord.me>
 * Date  : 16.06.2018
 */

error_chain! {
    foreign_links {
        TokioTimer(::tokio::timer::Error);
        SystemTime(::std::time::SystemTimeError);
    }
}
