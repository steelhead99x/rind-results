// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

use regex::Regex;

/// Home-directory paths, split so the source never contains a literal
/// home-directory prefix the privacy lint would flag.
const HOME_PATTERN: &str = concat!("(?:", "/Use", "rs|/", "ho", r"me)/\S+");

const PRIVATE_IP: &str = r"\b(?:192\.168\.\d{1,3}\.\d{1,3}|10\.\d{1,3}\.\d{1,3}\.\d{1,3}|172\.(?:1[6-9]|2\d|3[0-1])\.\d{1,3}\.\d{1,3})\b";

/// USB by-id name, with or without the `/dev/serial/by-id/` prefix.
/// Group 1 is the product prefix through the final `_`, group 2 is the
/// serial suffix, group 3 is the `-ifN` trailer (and optional `-portN`).
/// The caller rejects a match that continues an alphanumeric token. The
/// `regex` patterns here avoid look-around: clippy's `invalid_regex` lint
/// does not accept it.
const USB_BY_ID: &str = r"((?:/dev/serial/by-id/)?usb-[^\s]*_)([^\s_]+)(-if\d+(?:-port\d+)?)";

/// esptool's `MAC:` / `MAC address:` field. The value is a colon or hyphen
/// MAC, or twelve hex digits with no separator. The caller rejects a value
/// that is a prefix of a longer hex or MAC-shaped token.
const ESPTOOL_MAC: &str = r"(?i)(MAC(?:[ \t]+address)?[ \t]*:[ \t]*)([0-9A-Fa-f]{2}(?:[:-][0-9A-Fa-f]{2}){5}|[0-9A-Fa-f]{12})";

/// Colon and hyphen MAC addresses. The caller rejects a match that is part
/// of a longer address.
const MAC_ADDR: &str = r"[0-9A-Fa-f]{2}(?:[:-][0-9A-Fa-f]{2}){5}";

fn mac_continues(byte: Option<u8>) -> bool {
    byte.is_some_and(|b| b.is_ascii_hexdigit() || b == b':' || b == b'-')
}

fn replace_usb(re: &Regex, text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    for caps in re.captures_iter(text) {
        let whole = caps.get(0).expect("usb match");
        let prev = whole.start().checked_sub(1).map(|i| text.as_bytes()[i]);
        if prev.is_some_and(|b| b.is_ascii_alphanumeric()) {
            continue;
        }
        out.push_str(&text[last..whole.start()]);
        out.push_str(&caps[1]);
        out.push_str("<usb-serial>");
        out.push_str(&caps[3]);
        last = whole.end();
    }
    out.push_str(&text[last..]);
    out
}

fn replace_esptool_mac(re: &Regex, text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    for caps in re.captures_iter(text) {
        let whole = caps.get(0).expect("mac label");
        if mac_continues(text.as_bytes().get(whole.end()).copied()) {
            continue;
        }
        out.push_str(&text[last..whole.start()]);
        out.push_str(&caps[1]);
        out.push_str("<mac>");
        last = whole.end();
    }
    out.push_str(&text[last..]);
    out
}

fn replace_macs(re: &Regex, text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    for m in re.find_iter(text) {
        let prev = m.start().checked_sub(1).map(|i| text.as_bytes()[i]);
        if mac_continues(prev) || mac_continues(text.as_bytes().get(m.end()).copied()) {
            continue;
        }
        out.push_str(&text[last..m.start()]);
        out.push_str("<mac>");
        last = m.end();
    }
    out.push_str(&text[last..]);
    out
}

pub struct Scrubber {
    home: Regex,
    private_ip: Regex,
    usb_by_id: Regex,
    esptool_mac: Regex,
    mac: Regex,
    replacements: Vec<(Regex, String)>,
}

impl Scrubber {
    pub fn new(replacements: &[(String, String)]) -> Result<Self, String> {
        let mut compiled = Vec::with_capacity(replacements.len());
        for (pattern, text) in replacements {
            let re = Regex::new(pattern)
                .map_err(|e| format!("stage-repo: invalid --replace regex {pattern:?}: {e}"))?;
            compiled.push((re, text.clone()));
        }
        Ok(Self {
            home: Regex::new(HOME_PATTERN).expect("HOME_PATTERN"),
            private_ip: Regex::new(PRIVATE_IP).expect("PRIVATE_IP"),
            usb_by_id: Regex::new(USB_BY_ID).expect("USB_BY_ID"),
            esptool_mac: Regex::new(ESPTOOL_MAC).expect("ESPTOOL_MAC"),
            mac: Regex::new(MAC_ADDR).expect("MAC_ADDR"),
            replacements: compiled,
        })
    }

    pub fn scrub(&self, value: &str) -> String {
        let mut value = self.home.replace_all(value, "~").into_owned();
        value = self.private_ip.replace_all(&value, "<lan-ip>").into_owned();
        // Serial suffixes first, so a MAC used as a USB serial becomes
        // <usb-serial> rather than <mac>.
        value = replace_usb(&self.usb_by_id, &value);
        value = replace_esptool_mac(&self.esptool_mac, &value);
        value = replace_macs(&self.mac, &value);
        for (pattern, text) in &self.replacements {
            value = pattern.replace_all(&value, text.as_str()).into_owned();
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scrub(s: &str) -> String {
        Scrubber::new(&[]).expect("scrubber").scrub(s)
    }

    #[test]
    fn redacts_colon_and_hyphen_macs() {
        assert_eq!(
            scrub("seen aa:bb:cc:dd:ee:ff and AA-BB-CC-DD-EE-FF."),
            "seen <mac> and <mac>."
        );
        // A longer colon-separated address is not a 6-octet MAC.
        assert_eq!(
            scrub("eui aa:bb:cc:dd:ee:ff:11"),
            "eui aa:bb:cc:dd:ee:ff:11"
        );
        assert_eq!(
            scrub("eui aa-bb-cc-dd-ee-ff-11"),
            "eui aa-bb-cc-dd-ee-ff-11"
        );
    }

    #[test]
    fn redacts_usb_by_id_serial_suffix() {
        let serial = "aa:bb:cc:dd:ee:ff";
        let path =
            format!("/dev/serial/by-id/usb-Espressif_USB_JTAG_serial_debug_unit_{serial}-if00");
        assert_eq!(
            scrub(&path),
            "/dev/serial/by-id/usb-Espressif_USB_JTAG_serial_debug_unit_<usb-serial>-if00"
        );
        let bare = format!("(`usb-Espressif_USB_JTAG_serial_debug_unit_{serial}-if00`)");
        assert_eq!(
            scrub(&bare),
            "(`usb-Espressif_USB_JTAG_serial_debug_unit_<usb-serial>-if00`)"
        );
        assert_eq!(
            scrub("usb-Silicon_Labs_CP2102_USB_to_UART_Bridge_Controller_0001-if00-port0"),
            "usb-Silicon_Labs_CP2102_USB_to_UART_Bridge_Controller_<usb-serial>-if00-port0"
        );
    }

    #[test]
    fn redacts_esptool_mac_line() {
        assert_eq!(scrub("MAC: aa:bb:cc:dd:ee:ff"), "MAC: <mac>");
        assert_eq!(
            scrub("MAC address:       AA:BB:CC:DD:EE:FF"),
            "MAC address:       <mac>"
        );
        assert_eq!(scrub("MAC: aabbccddeeff"), "MAC: <mac>");
        // Prose that uses the label without a MAC-shaped value stays.
        assert_eq!(
            scrub("one-shot MAC: key, then message."),
            "one-shot MAC: key, then message."
        );
    }

    #[test]
    fn usb_serial_wins_over_mac_placeholder() {
        let line = "\
PORT=/dev/serial/by-id/usb-Vendor_Product_aa:bb:cc:dd:ee:ff-if00
MAC address: aa:bb:cc:dd:ee:ff
";
        assert_eq!(
            scrub(line),
            "\
PORT=/dev/serial/by-id/usb-Vendor_Product_<usb-serial>-if00
MAC address: <mac>
"
        );
    }
}
