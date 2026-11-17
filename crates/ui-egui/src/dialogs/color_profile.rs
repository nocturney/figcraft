use super::{ColorProfile, Context, FigApp, RichText, Tokens, json, regular, tr, trf, window};

/// File ▸ Color profile: pick sRGB or Display P3, then keep the colours' appearance (convert the
/// values) or keep the values (reinterpret them in the new space).
pub(super) fn color_profile_dialog(app: &mut FigApp, ctx: &Context) {
    let Some(mut chosen) = app.ui.color_profile else { return };
    let Ok(current) = app.session.doc().map(|d| d.color_profile) else {
        app.ui.color_profile = None;
        return;
    };
    let t = Tokens::get(ctx);
    let mut open = true;
    let mut apply = None;
    window(tr("Color profile"), ctx).open(&mut open).show(ctx, |ui| {
        ui.set_width(340.0);
        let rtl = crate::i18n::right_to_left();
        // Keep content-height rows: a vertical RTL layout can expand a dialog to screen height.
        let aligned_label = |ui: &mut egui::Ui, label: RichText| {
            if rtl {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(egui::Label::new(label).wrap());
                    });
                });
            } else {
                ui.label(label);
            }
        };
        aligned_label(ui, RichText::new(tr("Color values in this file are in this color space.")).color(t.text_secondary).font(regular(11.0)));
        ui.add_space(8.0);
        for p in ColorProfile::ALL {
            let label = if p == current { trf("{profile} (current)", &[("profile", &p.label())]) } else { p.label().to_string() };
            if rtl {
                // Give each RTL choice the full row, so the radio itself sits at the
                // right reading edge rather than in a content-width centered group.
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), 24.0),
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        if crate::widgets::radio(ui, chosen == p, &label) {
                            chosen = p;
                        }
                    },
                );
            } else if crate::widgets::radio(ui, chosen == p, &label) {
                chosen = p;
            }
        }
        ui.add_space(8.0);
        if chosen == current {
            aligned_label(ui, RichText::new(tr("Choose another profile to change it.")).color(t.text_secondary).font(regular(11.0)));
            return;
        }
        aligned_label(
            ui,
            RichText::new(trf(
                "Keep appearance converts every color value to {profile} so colors look the same. Keep values leaves the numbers as they are, so colors shift.",
                &[("profile", &chosen.label())],
            ))
            .font(regular(11.0)),
        );
        if !chosen.is_srgb() {
            ui.add_space(4.0);
            aligned_label(
                ui,
                RichText::new(tr("The canvas previews colors in sRGB; PNG, JPG and WebP exports that use the file's profile are tagged Display P3."))
                    .color(t.text_secondary)
                    .font(regular(11.0)),
            );
        }
        ui.add_space(12.0);
        let mut draw_actions = |ui: &mut egui::Ui| {
            if crate::widgets::button(ui, tr("Keep appearance"), true, true).clicked() {
                apply = Some(true);
            }
            if crate::widgets::button(ui, tr("Keep values"), false, true).clicked() {
                apply = Some(false);
            }
        };
        if rtl {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| draw_actions(ui));
            });
        } else {
            ui.horizontal(|ui| draw_actions(ui));
        }
    });
    app.ui.color_profile = open.then_some(chosen);
    if let Some(convert) = apply {
        app.run("file.colorProfile", json!({"profile": chosen.id(), "convert": convert}));
        app.ui.color_profile = None;
    }
}
