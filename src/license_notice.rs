use gloo_console::{group, group_end, log};

pub const LICENSE_NOTICE: &str = r#"
Realraum UI is free software: you can redistribute it and/or modify it under the terms
of the GNU Affero General Public License as published by the Free Software Foundation,
either version 3 of the License, or (at your option) any later version.

Realraum UI is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with Realraum UI.
If not, see https://www.gnu.org/licenses/.

"#;

pub fn log_license_notice() {
    let license_header = format!("Realraum UI v{} license notice", env!("CARGO_PKG_VERSION"));
    group!(license_header);
    log!(LICENSE_NOTICE);

    log!(
        "Licensed under the AGPLv3\n\
        Copyright © 2023 Tanja-4732."
    );

    log!("See the source code at https://github.com/realraum/realraum-frontend");

    group_end!();
}
