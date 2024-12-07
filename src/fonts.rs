use nannou_egui::{
    egui::{FontData, FontDefinitions, FontFamily, FontId, TextStyle},
    Egui,
};

pub fn set_fonts(egui: &mut Egui) {
    let ctx = &mut egui.ctx();

    let mut fonts = FontDefinitions::default();

    let stix_text = [
        (
            "STIXTwoText-Bold",
            include_bytes!("../fonts/STIX_Two/STIXTwoText-Bold.ttf").as_slice(),
        ),
        (
            "STIXTwoText-BoldItalic",
            include_bytes!("../fonts/STIX_Two/STIXTwoText-BoldItalic.ttf").as_slice(),
        ),
        (
            "STIXTwoText-Italic",
            include_bytes!("../fonts/STIX_Two/STIXTwoText-Italic.ttf").as_slice(),
        ),
        (
            "STIXTwoText-Medium",
            include_bytes!("../fonts/STIX_Two/STIXTwoText-Medium.ttf").as_slice(),
        ),
        (
            "STIXTwoText-MediumItalic",
            include_bytes!("../fonts/STIX_Two/STIXTwoText-MediumItalic.ttf").as_slice(),
        ),
        (
            "STIXTwoText-Regular",
            include_bytes!("../fonts/STIX_Two/STIXTwoText-Regular.ttf").as_slice(),
        ),
        (
            "STIXTwoText-SemiBold",
            include_bytes!("../fonts/STIX_Two/STIXTwoText-SemiBold.ttf").as_slice(),
        ),
        (
            "STIXTwoText-SemiBoldItalic",
            include_bytes!("../fonts/STIX_Two/STIXTwoText-SemiBoldItalic.ttf").as_slice(),
        ),
    ];

    for (name, font) in &stix_text {
        fonts
            .font_data
            .insert(name.to_string(), FontData::from_static(font));
    }

    fonts.font_data.insert(
        "STIXTwoMath-Regular".to_string(),
        FontData::from_static(include_bytes!("../fonts/STIX_Two/STIXTwoMath-Regular.ttf")),
    );

    fonts
        .families
        .entry(FontFamily::Name("STIXTwoText".into()))
        .or_default()
        .extend(stix_text.iter().map(|(name, _)| name.to_string()));

    fonts
        .families
        .entry(FontFamily::Name("STIXTwoMath".into()))
        .or_default()
        .push("STIXTwoMath-Regular".to_string());

    ctx.set_fonts(fonts);

    let mut style = (*ctx.style()).clone();

    style.text_styles.insert(
        TextStyle::Name("STIXTwoText".into()),
        FontId::new(16.0, FontFamily::Name("STIXTwoText".into())),
    );

    style.text_styles.insert(
        TextStyle::Name("STIXTwoMath".into()),
        FontId::new(16.0, FontFamily::Name("STIXTwoMath".into())),
    );

    ctx.set_style(style);
}
