//! Logos compiled into the binary.
//!
//! App tiles, distro marks, and the Brave Origin tile use SVG markup via
//! `include_str!`. Icons are vendored from dashboard-icons (Apache-2.0),
//! Simple Icons (CC0), Flathub / upstream app icon SVGs, or Flathub PNG
//! icons wrapped as data-URI SVGs. See `assets/logos/NOTICE`. Nothing under
//! `assets/` is read from disk at runtime.

use eframe::egui::ColorImage;

/// Distro SVG markup keyed the same way the former header chips looked them up.
#[cfg_attr(not(test), allow(dead_code))]
pub const DISTRO_SVGS: &[(&str, &str)] = &[
    ("arch", include_str!("../assets/distros/arch.svg")),
    ("debian", include_str!("../assets/distros/debian.svg")),
    ("fedora", include_str!("../assets/distros/fedora.svg")),
];

/// Chris's Brave Origin tile: white outlined lion on a dark rounded square.
/// Do not reuse the orange Brave Browser mark.
pub const BRAVE_ORIGIN_SVG: &str = include_str!("../assets/logos/brave-origin.svg");

/// Colorful app-tile SVGs keyed by icon slug.
pub const APP_SVGS: &[(&str, &str)] = &[
    ("audacity", include_str!("../assets/logos/audacity.svg")),
    ("bitwarden", include_str!("../assets/logos/bitwarden.svg")),
    ("bottles", include_str!("../assets/logos/bottles.svg")),
    ("boxes", include_str!("../assets/logos/boxes.svg")),
    ("brave", include_str!("../assets/logos/brave.svg")),
    ("discord", include_str!("../assets/logos/discord.svg")),
    (
        "dolphin-emu",
        include_str!("../assets/logos/dolphin-emu.svg"),
    ),
    (
        "extension-manager",
        include_str!("../assets/logos/extension-manager.svg"),
    ),
    ("firefox", include_str!("../assets/logos/firefox.svg")),
    ("flatseal", include_str!("../assets/logos/flatseal.svg")),
    ("gear-lever", include_str!("../assets/logos/gear-lever.svg")),
    ("gimp", include_str!("../assets/logos/gimp.svg")),
    (
        "google-chrome",
        include_str!("../assets/logos/google-chrome.svg"),
    ),
    ("gparted", include_str!("../assets/logos/gparted.svg")),
    ("heroic", include_str!("../assets/logos/heroic.svg")),
    ("htop", include_str!("../assets/logos/htop.svg")),
    (
        "libreoffice",
        include_str!("../assets/logos/libreoffice.svg"),
    ),
    ("librewolf", include_str!("../assets/logos/librewolf.svg")),
    ("localsend", include_str!("../assets/logos/localsend.svg")),
    ("lutris", include_str!("../assets/logos/lutris.svg")),
    (
        "microsoft-edge",
        include_str!("../assets/logos/microsoft-edge.svg"),
    ),
    (
        "mission-center",
        include_str!("../assets/logos/mission-center.svg"),
    ),
    ("mpv", include_str!("../assets/logos/mpv.svg")),
    ("obsidian", include_str!("../assets/logos/obsidian.svg")),
    ("obs-studio", include_str!("../assets/logos/obs-studio.svg")),
    ("onlyoffice", include_str!("../assets/logos/onlyoffice.svg")),
    ("opera", include_str!("../assets/logos/opera.svg")),
    ("ppsspp", include_str!("../assets/logos/ppsspp.svg")),
    (
        "prism-launcher",
        include_str!("../assets/logos/prism-launcher.svg"),
    ),
    ("proton-vpn", include_str!("../assets/logos/proton-vpn.svg")),
    ("protonplus", include_str!("../assets/logos/protonplus.svg")),
    (
        "protonup-qt",
        include_str!("../assets/logos/protonup-qt.svg"),
    ),
    (
        "pycharm-community",
        include_str!("../assets/logos/pycharm-community.svg"),
    ),
    (
        "qbittorrent",
        include_str!("../assets/logos/qbittorrent.svg"),
    ),
    ("retroarch", include_str!("../assets/logos/retroarch.svg")),
    ("signal", include_str!("../assets/logos/signal.svg")),
    ("slack", include_str!("../assets/logos/slack.svg")),
    ("sober", include_str!("../assets/logos/sober.svg")),
    ("spotify", include_str!("../assets/logos/spotify.svg")),
    ("steam", include_str!("../assets/logos/steam.svg")),
    ("stremio", include_str!("../assets/logos/stremio.svg")),
    ("telegram", include_str!("../assets/logos/telegram.svg")),
    (
        "thunderbird",
        include_str!("../assets/logos/thunderbird.svg"),
    ),
    (
        "visual-studio-code",
        include_str!("../assets/logos/visual-studio-code.svg"),
    ),
    ("vivaldi", include_str!("../assets/logos/vivaldi.svg")),
    ("vlc", include_str!("../assets/logos/vlc.svg")),
    (
        "zen-browser",
        include_str!("../assets/logos/zen-browser.svg"),
    ),
    (
        "zen-browser-dark",
        include_str!("../assets/logos/zen-browser-dark.svg"),
    ),
];

/// Formerly Simple Icons monochrome tiles. Kept empty now that Flathub /
/// brand color icons are vendored into [`APP_SVGS`].
pub const MONO_SVGS: &[(&str, &str)] = &[];

pub fn is_monochrome_icon(key: &str) -> bool {
    MONO_SVGS.iter().any(|(slug, _)| *slug == key)
}

pub struct Raster {
    pub size: [usize; 2],
    pub rgba: Vec<u8>,
}

fn svg_parse_options() -> usvg::Options<'static> {
    let mut options = usvg::Options::default();
    options.image_href_resolver.resolve_string = Box::new(|_, _| None);
    options
}

/// Rasterize SVG markup that is already in memory. This never opens a path.
pub fn rasterize_svg_markup(svg: &str, size: u32) -> Option<Raster> {
    let options = svg_parse_options();
    let tree = usvg::Tree::from_str(svg, &options).ok()?;
    let mut pixmap = tiny_skia::Pixmap::new(size, size)?;
    let tree_size = tree.size();
    let scale = (size as f32 / tree_size.width()).min(size as f32 / tree_size.height());
    let tx = (size as f32 - tree_size.width() * scale) / 2.0;
    let ty = (size as f32 - tree_size.height() * scale) / 2.0;
    let transform = tiny_skia::Transform::from_translate(tx, ty).pre_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    Some(Raster {
        size: [size as usize, size as usize],
        rgba: pixmap.data().to_vec(),
    })
}

pub fn color_image_from_raster(raster: &Raster) -> ColorImage {
    ColorImage::from_rgba_unmultiplied(raster.size, &raster.rgba)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn assert_markup(name: &str, svg: &str) {
        assert!(
            svg.contains("<svg"),
            "{name} markup is missing an <svg> tag"
        );
        assert!(
            !svg.starts_with("assets/"),
            "{name} looks like a filesystem path rather than markup"
        );
    }

    fn assert_raster(name: &str, svg: &str) {
        let raster = rasterize_svg_markup(svg, 64)
            .unwrap_or_else(|| panic!("{name} SVG failed to rasterize"));
        assert_eq!(raster.size, [64, 64], "{name}");
        assert_eq!(raster.rgba.len(), 64 * 64 * 4, "{name}");
        assert!(
            raster.rgba.chunks(4).any(|px| px[3] != 0),
            "{name} raster was fully transparent"
        );
    }

    #[test]
    fn distro_svgs_are_embedded_markup() {
        assert_eq!(DISTRO_SVGS.len(), 3);
        for (name, svg) in DISTRO_SVGS {
            assert_markup(name, svg);
            assert_raster(name, svg);
        }
    }

    #[test]
    fn brave_origin_svg_is_chris_markup() {
        assert_markup("brave-origin", BRAVE_ORIGIN_SVG);
        assert_raster("brave-origin", BRAVE_ORIGIN_SVG);
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let on_disk = std::fs::read_to_string(root.join("assets/logos/brave-origin.svg")).unwrap();
        assert_eq!(BRAVE_ORIGIN_SVG, on_disk.as_str());
    }

    #[test]
    fn app_svgs_are_embedded_dashboardicons_markup() {
        let keys: Vec<&str> = APP_SVGS.iter().map(|(key, _)| *key).collect();
        assert!(keys.contains(&"brave"));
        assert!(keys.contains(&"zen-browser"));
        assert!(keys.contains(&"zen-browser-dark"));
        assert!(!keys.contains(&"brave-origin"));
        for (name, svg) in APP_SVGS {
            assert_markup(name, svg);
            assert_raster(name, svg);
        }
    }

    #[test]
    fn mono_svgs_are_retired_in_favor_of_color_flathub_icons() {
        assert!(
            MONO_SVGS.is_empty(),
            "monochrome Simple Icons tiles were replaced with Flathub color icons"
        );
        for expected in [
            "vlc",
            "lutris",
            "heroic",
            "obs-studio",
            "mpv",
            "htop",
            "pycharm-community",
        ] {
            assert!(
                APP_SVGS.iter().any(|(key, _)| *key == expected),
                "{expected} should live in APP_SVGS"
            );
            assert!(!is_monochrome_icon(expected), "{expected}");
        }
        assert!(!is_monochrome_icon("brave"));
    }

    #[test]
    fn include_str_matches_checkout_files() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        for (name, embedded) in DISTRO_SVGS {
            let path = root.join("assets/distros").join(format!("{name}.svg"));
            let on_disk = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            assert_eq!(*embedded, on_disk.as_str(), "{name}");
        }
        for (name, embedded) in APP_SVGS.iter().chain(MONO_SVGS.iter()) {
            let path = root.join("assets/logos").join(format!("{name}.svg"));
            let on_disk = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            assert_eq!(*embedded, on_disk.as_str(), "{name}");
        }
    }

    #[test]
    fn assets_icons_dir_is_gone() {
        let icons = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/icons");
        assert!(!icons.exists(), "assets/icons must stay removed");
    }

    #[test]
    fn app_rasters_are_not_shipped() {
        let logos = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/logos");
        let pngs: Vec<_> = std::fs::read_dir(&logos)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("png"))
            .collect();
        assert!(
            pngs.is_empty(),
            "app tile PNGs must not be shipped: {pngs:?}"
        );
        let prod = include_str!("logos.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("logos module");
        assert!(!prod.contains("APP_PNGS"));
        assert!(!prod.contains("include_bytes!"));
        assert!(!prod.contains(".png"));
    }

    #[test]
    fn brave_origin_is_not_the_orange_brave_mark() {
        let brave = APP_SVGS
            .iter()
            .find(|(key, _)| *key == "brave")
            .map(|(_, svg)| *svg)
            .expect("brave svg");
        assert_ne!(BRAVE_ORIGIN_SVG, brave);
        assert!(
            !BRAVE_ORIGIN_SVG.to_ascii_lowercase().contains("#fb542b"),
            "Origin SVG must not use the orange Brave fill"
        );
        let origin = rasterize_svg_markup(BRAVE_ORIGIN_SVG, 32).unwrap();
        let brave_img = rasterize_svg_markup(brave, 32).unwrap();
        assert_ne!(origin.rgba, brave_img.rgba);
    }

    #[test]
    fn former_letter_fallback_apps_now_ship_icons() {
        let logos = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/logos");
        let keys: Vec<&str> = APP_SVGS
            .iter()
            .chain(MONO_SVGS.iter())
            .map(|(key, _)| *key)
            .collect();
        for expected in [
            "bottles",
            "boxes",
            "protonup-qt",
            "gparted",
            "localsend",
            "sober",
            "flatseal",
            "prism-launcher",
            "retroarch",
            "extension-manager",
            "dolphin-emu",
            "ppsspp",
            "gear-lever",
            "protonplus",
            "mission-center",
        ] {
            assert!(keys.contains(&expected), "{expected}");
            assert!(
                logos.join(format!("{expected}.svg")).exists(),
                "{expected}.svg"
            );
        }
    }

    #[test]
    fn notice_attributes_vendored_collections() {
        let notice = include_str!("../assets/logos/NOTICE");
        assert!(notice.contains("Apache License 2.0"));
        assert!(notice.contains("dashboard-icons"));
        assert!(notice.contains("Simple Icons"));
        assert!(notice.contains("brave-origin.svg"));
        assert!(notice.contains("protonplus.svg"));
        assert!(notice.contains("Flathub"));
        assert!(notice.contains("localsend.svg"));
        assert!(notice.contains("bottles.svg"));
    }

    #[test]
    fn rasterize_ignores_external_file_hrefs() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 8 8">
            <image href="/nonexistent/toolbox-logo.svg" width="8" height="8"/>
            <rect x="1" y="1" width="6" height="6" fill="#ff0000"/>
        </svg>"##;
        let raster = rasterize_svg_markup(svg, 8).expect("markup with a missing href");
        assert!(raster.rgba.chunks(4).any(|px| px[3] != 0));
    }

    #[test]
    fn rasterize_accepts_embedded_png_data_uris() {
        // 1x1 red PNG
        let svg = concat!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="8" height="8" viewBox="0 0 8 8">"##,
            r##"<image width="8" height="8" href="data:image/png;base64,"##,
            "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==",
            r##""/></svg>"##,
        );
        let raster = rasterize_svg_markup(svg, 8).expect("png data-uri svg");
        assert!(
            raster.rgba.chunks(4).any(|px| px[3] != 0),
            "embedded PNG should paint opaque pixels"
        );
    }
}
